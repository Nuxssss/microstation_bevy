use bevy::asset::Assets;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct RsiMeta {
    pub size: RsiSize,
    pub states: Vec<RsiStateMeta>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct RsiSize {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RsiStateMeta {
    pub name: String,
    #[serde(default = "default_dirs")]
    pub directions: u8,
    #[serde(default)]
    pub delays: Vec<Vec<f32>>,
}

fn default_dirs() -> u8 { 1 }

#[derive(Debug, Clone)]
pub struct RsiStateHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct RsiRegistry {
    fs_base: PathBuf,
    metas: HashMap<String, Option<RsiMeta>>,
    cache: HashMap<String, HashMap<String, RsiStateHandles>>,
}

impl RsiRegistry {
    pub fn new(assets_base: &str) -> Self {
        let mut fs_base = std::env::current_dir().unwrap_or_default();
        fs_base.push(assets_base);
        fs_base.push("Textures");
        info!("[RSI] FS root: {:?}", fs_base);
        Self { fs_base, metas: HashMap::new(), cache: HashMap::new() }
    }

    pub fn get_first_state(&mut self, rsi_id: &str) -> Option<String> {
        let meta = self.metas.entry(rsi_id.to_string()).or_insert_with(|| {
            let path = self.fs_base.join(rsi_id).join("meta.json");
            match fs::read_to_string(&path) {
                Ok(d) => serde_json::from_str(&d).ok(),
                Err(_) => None
            }
        });
        meta.as_ref().and_then(|m| m.states.first().map(|s| s.name.clone()))
    }

    pub fn get_handles(
        &mut self,
        rsi_id: &str,
        state: &str,
        layouts: &mut Assets<TextureAtlasLayout>,
        server: &AssetServer,
    ) -> Option<RsiStateHandles> {
        if let Some(h) = self.cache.get(rsi_id).and_then(|m| m.get(state)) {
            return Some(h.clone());
        }

        let meta = self.metas.entry(rsi_id.to_string()).or_insert_with(|| {
            let path = self.fs_base.join(rsi_id).join("meta.json");
            match fs::read_to_string(&path) {
                Ok(d) => serde_json::from_str(&d).ok(),
                Err(e) => { error!("[RSI] meta.json IO: {} | {}", path.display(), e); None }
            }
        });

        let Some(meta) = meta else { return None; };
        let Some(st) = meta.states.iter().find(|s| s.name == state) else {
            error!("[RSI] meta.json missing or state '{}' not found", state);
            return None;
        };

        let frames = st.delays.first().map(|d| d.len()).unwrap_or(1);
        let cols = (st.directions as u32) * frames as u32;
        let size = UVec2::new(meta.size.x, meta.size.y);
        let layout = layouts.add(TextureAtlasLayout::from_grid(size, cols, 1, None, None));

        let img_path = format!("Textures/{}/{}.png", rsi_id, st.name);
        let img_h = server.load(&img_path);

        let handles = RsiStateHandles { image: img_h, layout };
        self.cache.entry(rsi_id.to_string()).or_default().insert(state.to_string(), handles.clone());
        Some(handles)
    }
}

#[derive(Resource)]
pub struct ErrorSprite {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub struct RsiPlugin;
impl Plugin for RsiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RsiRegistry::new("Resources"));
        app.add_systems(Startup, init_error_sprite);
    }
}

fn init_error_sprite(mut commands: Commands, mut layouts: ResMut<Assets<TextureAtlasLayout>>, server: Res<AssetServer>) {
    let img = server.load("Textures/error.rsi/error.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None));
    commands.insert_resource(ErrorSprite { image: img, layout });
    info!("[RSI] Error sprite handles initialized");
}