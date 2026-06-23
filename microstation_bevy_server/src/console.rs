use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use clap::{ColorChoice, Parser, Subcommand};
use microstation_bevy_shared::prototypes::PrototypeManager;
use microstation_bevy_shared::world::Position;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Add;

pub struct ConsolePlugin {
    pub port: u16,
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        let listener =
            TcpListener::bind(format!("127.0.0.1:{}", self.port)).expect("console bind failed");
        listener.set_nonblocking(true).unwrap();

        info!("Debug console on 127.0.0.1:{}", self.port);

        app.insert_resource(ConsoleServer {
            listener,
            clients: vec![],
            next_client_id: 0,
        });
        // ConsoleQuery is a trigger-style Event, not a buffered Message,
        // so no add_event::<T>() registration is needed — add_observer is enough.
        app.add_observer(handle_console_query);
        app.add_systems(Update, poll_console);
    }
}

#[derive(Resource)]
struct ConsoleServer {
    listener: TcpListener,
    clients: Vec<ConsoleClient>,
    next_client_id: usize,
}

struct ConsoleClient {
    id: usize,
    stream: TcpStream,
    in_buf: Vec<u8>,
    out_buf: VecDeque<u8>,
    dead: bool,
}

impl ConsoleClient {
    fn new(id: usize, stream: TcpStream) -> Self {
        Self {
            id,
            stream,
            in_buf: Vec::new(),
            out_buf: VecDeque::new(),
            dead: false,
        }
    }

    /// Reads available bytes, returns complete `\n`-terminated lines.
    /// Decoded per line, not per read() chunk — avoids mangling multibyte
    /// UTF-8 (e.g. Cyrillic) split across two reads.
    fn read_lines(&mut self) -> Vec<String> {
        let mut tmp = [0u8; 1024];

        loop {
            match self.stream.read(&mut tmp) {
                Ok(0) => {
                    self.dead = true;
                    break;
                }
                Ok(n) => self.in_buf.extend_from_slice(&tmp[..n]),
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => {
                    self.dead = true;
                    break;
                }
            }
        }

        let mut lines = vec![];
        while let Some(pos) = self.in_buf.iter().position(|&b| b == b'\n') {
            let raw: Vec<u8> = self.in_buf.drain(..=pos).collect();
            // `raw` is a complete, self-contained line, so from_utf8_lossy
            // here can't split a multi-byte character — unlike decoding
            // each raw read() chunk independently, as the original did.
            let line = String::from_utf8_lossy(&raw[..raw.len() - 1])
                .trim_end_matches('\r')
                .trim()
                .to_string();
            if !line.is_empty() {
                lines.push(line);
            }
        }

        lines
    }

    /// Buffers text until `flush_writes` sends it.
    fn queue_write(&mut self, text: &str) {
        self.out_buf.extend(text.as_bytes());
    }

    /// Writes as much of `out_buf` as the socket will currently accept.
    /// Whatever doesn't fit (WouldBlock on a full send buffer) stays queued
    /// for the next call instead of being silently discarded mid-line.
    fn flush_writes(&mut self) {
        while !self.out_buf.is_empty() {
            let chunk = self.out_buf.make_contiguous();
            match self.stream.write(chunk) {
                Ok(0) => {
                    self.dead = true;
                    return;
                }
                Ok(n) => {
                    self.out_buf.drain(..n);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => return,
                Err(_) => {
                    self.dead = true;
                    return;
                }
            }
        }
    }
}

/// Fired for every console command, immediate or world-dependent alike.
/// Routing *all* replies through this single trigger (see `queue_reply` and
/// `handle_command`) means they're written back in exactly the order the
/// commands were issued, instead of immediate replies jumping ahead of
/// deferred ones within the same batch.
#[derive(Event)]
struct ConsoleQuery {
    client: usize,
    kind: ConsoleQueryKind,
}

enum ConsoleQueryKind {
    /// Reply text already computed; the observer just has to write it.
    Immediate(String),
    ListEntities,
    EntityInfo(Entity),
}

#[derive(Parser)]
#[command(
    no_binary_name = true,
    disable_version_flag = true,
    disable_help_flag = true,
    color = ColorChoice::Never,
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Spawn a prototype, optionally at a position
    Spawn {
        /// Prototype id to spawn
        id: String,
        /// X position (must be given together with Y)
        x: Option<i32>,
        /// Y position
        y: Option<i32>,
    },
    /// List all entities currently in the world
    Entities,
    /// Show info about one entity (id is the number printed by `entities`/`spawn`)
    Entity { id: u64 },
    /// Clear the terminal screen
    Clear,
}

fn poll_console(
    mut server: ResMut<ConsoleServer>,
    mut commands: Commands,
    prototypes: Res<PrototypeManager>,
) {
    loop {
        match server.listener.accept() {
            Ok((stream, addr)) => {
                if let Err(e) = stream.set_nonblocking(true) {
                    warn!("Console: failed to configure client socket: {e}");
                    continue;
                }
                let id = server.next_client_id;
                server.next_client_id += 1;
                let mut client = ConsoleClient::new(id, stream);
                client.queue_write("> ");
                client.flush_writes();
                info!("Console client connected: {addr} (#{id})");
                server.clients.push(client);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => break,
            Err(e) => {
                warn!("Console accept error: {e}");
                break;
            }
        }
    }

    for client in server.clients.iter_mut() {
        let lines = client.read_lines();
        let client_id = client.id;

        for line in lines {
            handle_command(&line, client_id, &mut commands, &prototypes);
        }

        // Push out anything already queued: last frame's deferred replies,
        // or bytes that didn't fit on an earlier attempt.
        client.flush_writes();
    }

    server.clients.retain(|c| !c.dead);
}

fn handle_command(
    line: &str,
    client_id: usize,
    commands: &mut Commands,
    prototypes: &PrototypeManager,
) {
    let args = match shlex::split(line) {
        Some(a) => a,
        None => {
            queue_reply(commands, client_id, "error: unbalanced quotes".to_string());
            return;
        }
    };

    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        // Covers both real parse errors and help/--help output —
        // clap puts both into the same Err variant.
        Err(e) => {
            queue_reply(commands, client_id, e.to_string().trim_end().to_string());
            return;
        }
    };

    match cli.command {
        Cmd::Clear => queue_reply(commands, client_id, "\x1B[2J\x1B[H".to_string()),

        Cmd::Spawn { id, x, y } => {
            // x/y are positional, so clap's `requires` can't actually catch
            // "only one given" here — it fills left-to-right regardless.
            // Check explicitly instead of silently dropping a half-given position.
            let pos = match (x, y) {
                (Some(x), Some(y)) => Some(Position(IVec2::new(x, y))),
                (None, None) => None,
                _ => {
                    queue_reply(
                        commands,
                        client_id,
                        "error: x and y must be given together".to_string(),
                    );
                    return;
                }
            };
            let text = match prototypes.spawn_entity(&id, pos, commands) {
                Some(entity) => format!("spawned {id} -> {entity:?} (id={})", entity.to_bits()),
                None => format!("error: unknown prototype '{id}'"),
            };
            queue_reply(commands, client_id, text);
        }

        Cmd::Entities => {
            commands.trigger(ConsoleQuery {
                client: client_id,
                kind: ConsoleQueryKind::ListEntities,
            });
        }

        Cmd::Entity { id } => {
            commands.trigger(ConsoleQuery {
                client: client_id,
                kind: ConsoleQueryKind::EntityInfo(Entity::from_bits(id)),
            });
        }
    }
}

/// Queues an already-known reply through the same trigger mechanism as the
/// world-dependent commands, so output ordering stays consistent no matter
/// which kind of command produced it (see the note on `ConsoleQuery`).
fn queue_reply(commands: &mut Commands, client_id: usize, text: String) {
    commands.trigger(ConsoleQuery {
        client: client_id,
        kind: ConsoleQueryKind::Immediate(text),
    });
}

/// Resolves every ConsoleQuery — both the ones needing live world access and
/// the ones that don't — and writes the reply back to the right client.
/// Because every reply goes through this one observer, and Bevy applies
/// queued commands/triggers in the order they were issued, several commands
/// sent in one batch from a client come back in the same order they were sent.
fn handle_console_query(trigger: On<ConsoleQuery>, mut world: DeferredWorld) {
    // `World::query` needs `&mut World` (it may register new component types),
    // which `DeferredWorld` doesn't expose. `try_query` only needs `&World`
    // (deref-forwarded) and returns `None` only if some component in `D` isn't
    // registered yet — `Entity` alone never has that problem, so `unwrap` is safe.
    let mut entities = world.try_query::<Entity>().unwrap();
    let event = trigger.event();

    let text = match &event.kind {
        ConsoleQueryKind::Immediate(text) => text.clone(),

        ConsoleQueryKind::ListEntities => {
            let mut out = format!("=== Entities ({}) ===\n", entities.iter(&world).count());
            for e in entities.iter(&world) {
                let name = world
                    .entity(e)
                    .get::<Name>()
                    .map(|n| n.as_str())
                    .unwrap_or("<unnamed>");
                out.push_str(format!(" {name} | id={} | {e:?}\n", e.to_bits()).as_str())
            }
            out
        }

        ConsoleQueryKind::EntityInfo(target) => match entities.get(&world, *target) {
            Ok(entity) => {
                let mut out = format!("=== Entity {entity:?} (id={}) ===\n", entity.to_bits());

                let display_name = world
                    .entity(entity)
                    .get::<Name>()
                    .map(|n| n.as_str())
                    .unwrap_or("<unnamed>");
                out.push_str(&format!("  name: {display_name}\n"));

                for c_info in world.inspect_entity(entity).unwrap() {
                    let c_name = c_info.name();

                    // `inspect_entity` only gives us metadata (ComponentInfo), not the
                    // value. To print the actual value we go through bevy_reflect:
                    // requires `#[derive(Reflect)]` on the component AND
                    // `app.register_type::<T>()` having been called for it. Anything
                    // that isn't (e.g. raw third-party or unregistered types) falls
                    // back to just the name.
                    let value = c_info
                        .type_id()
                        .and_then(|tid| world.get_reflect(entity, tid).ok());

                    match value {
                        Some(reflected) => out.push_str(&format!("  {reflected:#?}\n")),
                        None => out.push_str(&format!("  {c_name} (not reflected)\n")),
                    }
                }

                out
            }
            Err(_) => format!("error: no such entity (id={})", target.to_bits()),
        },
    };

    let Some(mut server) = world.get_resource_mut::<ConsoleServer>() else {
        return;
    };
    if let Some(client) = server.clients.iter_mut().find(|c| c.id == event.client) {
        client.queue_write(&text);
        client.queue_write("\n> ");
        client.flush_writes();
    }
}
