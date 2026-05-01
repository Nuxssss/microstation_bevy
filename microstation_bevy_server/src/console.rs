use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};

use bevy::prelude::*;
use microstation_bevy_shared::prototypes::PrototypeManager;
use microstation_bevy_shared::world::Position;

pub struct ConsolePlugin {
    pub port: u16,
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("console bind failed");
        listener.set_nonblocking(true).unwrap();

        info!("Debug console on 127.0.0.1:{}", self.port);

        app.insert_resource(ConsoleServer {
            listener,
            clients: vec![],
        });
        app.add_systems(Update, poll_console);
    }
}

#[derive(Resource)]
struct ConsoleServer {
    listener: TcpListener,
    clients: Vec<ConsoleClient>,
}

struct ConsoleClient {
    stream: TcpStream,
    buf: String,
}

impl ConsoleClient {
    fn new(stream: TcpStream) -> Self {
        Self { stream, buf: String::new() }
    }

    fn read_lines(&mut self) -> (Vec<String>, bool) {
        let mut tmp = [0u8; 1024];
        let mut lines = vec![];
        let mut disconnected = false;

        loop {
            match self.stream.read(&mut tmp) {
                Ok(0) => { disconnected = true; break; }
                Ok(n) => {
                    self.buf.push_str(&String::from_utf8_lossy(&tmp[..n]));
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => { disconnected = true; break; }
            }
        }

        while let Some(pos) = self.buf.find('\n') {
            let line = self.buf[..pos].trim_end_matches('\r').trim().to_string();
            self.buf.drain(..=pos);
            if !line.is_empty() {
                lines.push(line);
            }
        }

        (lines, disconnected)
    }
}

fn poll_console(
    mut server: ResMut<ConsoleServer>,
    mut commands: Commands,
    prototypes: Res<PrototypeManager>,
) {
    loop {
        match server.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true).unwrap();
                let mut client = ConsoleClient::new(stream);
                let _ = client.stream.write_all(b"> ");
                info!("Console client connected: {addr}");
                server.clients.push(client);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => break,
            Err(e) => { warn!("Console accept error: {e}"); break; }
        }
    }

    let mut to_remove = vec![];

    for (i, client) in server.clients.iter_mut().enumerate() {
        let (lines, disconnected) = client.read_lines();

        for line in lines {
            let response = handle_command(&line, &mut commands, &prototypes);
            let _ = client.stream.write_all(format!("{response}\n> ").as_bytes());
        }

        if disconnected {
            to_remove.push(i);
        }
    }

    for i in to_remove.into_iter().rev() {
        info!("Console client disconnected");
        server.clients.remove(i);
    }
}

fn handle_command(line: &str, commands: &mut Commands, prototypes: &PrototypeManager) -> String {
    let args = match shlex::split(line) {
        Some(a) if !a.is_empty() => a,
        _ => return "error: invalid input".to_string(),
    };

    match args.as_slice() {
        [cmd] if cmd == "help" => {
            "commands: spawn <id> [x] [y] | help".to_string()
        }
        [cmd, id] if cmd == "spawn" => {
            match prototypes.spawn_entity(id, None, commands) {
                Some(entity) => format!("spawned {id} -> {entity:?}"),
                None => format!("error: unknown prototype '{id}'"),
            }
        }
        [cmd, id, x, y] if cmd == "spawn" => {
            let (Ok(x), Ok(y)) = (x.parse(), y.parse()) else {
                return "coords must be int".to_string();
            };
            match prototypes.spawn_entity(id, Some(Position(IVec2::new(x, y))), commands) {
                Some(entity) => format!("spawned {id} -> {entity:?}"),
                None => format!("error: unknown prototype '{id}'"),
            }
        }
        [cmd, ..] => {
            format!("error: unknown command '{cmd}'")
        }
        _ => "error: empty command".to_string(),
    }
}