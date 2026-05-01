
// Дальше бога нет. Только макросы

use bevy::color::{Color, ColorToComponents};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_inline_default::serde_inline_default;
use serde_nested_with::serde_nested;
use crate::draw_depth::{DrawDepth, deserialize_depth_int};
use crate::helpers::{force_string};
use crate::color::deserialize_color;

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize, Serialize, Default, Component)]
#[serde(rename_all = "camelCase")]
#[require(Transform, Visibility)]
pub struct ComplexSprite {
    /// Базовый RSI для всех слоёв
    #[serde(rename = "sprite")]
    pub rsi_path: Option<String>,
    #[serde(default, rename = "drawdepth", deserialize_with = "deserialize_depth_int")]
    pub draw_depth: DrawDepth,
    #[serde_inline_default(true)]
    pub visible: bool,
    #[serde(default, deserialize_with = "deserialize_color")]
    pub color: Option<Color>, // TODO Заменить на свой color, т.к. иначе серверная часть тянет рендер беви (пиздец)
    #[serde(rename = "noRot", default)]
    pub no_rotation: bool,
    #[serde_inline_default(true)]
    pub snap_cardinals: bool,
    pub state: Option<String>,
    pub texture: Option<String>,
    #[serde(default)]
    pub layers: Vec<Layer>,
    #[serde_inline_default(true)]
    pub granular_layers_rendering: bool,
}

impl ComplexSprite {
    pub fn normalize(&mut self) {
        if self.layers.is_empty() && (self.rsi_path.is_some() || self.state.is_some() || self.color.is_some()) {
            self.layers.push(Layer {
                rsi_path: self.rsi_path.take(),
                state: self.state.take(),
                color: self.color.take(),
                visible: self.visible,
                scale: None,
                offset: None,
                rotation: None,
                ..Default::default()
            });
        }
    }
}

#[serde_inline_default]
#[serde_nested]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    #[serde_nested(sub = "String", serde(deserialize_with = "force_string"))]
    #[serde(default)]
    pub state: Option<String>,
    #[serde(rename = "sprite")]
    pub rsi_path: Option<String>,
    #[serde(rename = "texture")]
    pub texture_path: Option<String>,
    #[serde(default)]
    pub map: Vec<String>,
    #[serde_inline_default(true)]
    pub visible: bool,
    #[serde(default, deserialize_with = "deserialize_color")] //TODO заменить этот нейровысер на serde_nested
    pub color: Option<Color>,
    pub shader: Option<String>,
    pub scale: Option<Vec2Yaml>,
    pub rotation: Option<f32>,
    pub offset: Option<Vec2Yaml>,
    #[serde(default)]
    pub cycle: bool,
    #[serde_inline_default(true)]
    pub loop_anim: bool,

    // runtime состояние — не из YAML, заполняется при загрузке
    #[serde(skip)]
    pub auto_animated: bool,
    #[serde(skip)]
    pub reversed: bool,
    #[serde(skip)]
    pub animation_time: f32,
    #[serde(skip)]
    pub animation_time_left: f32,
    #[serde(skip)]
    pub frame: usize,
    #[serde(skip)]
    pub dir_offset: DirectionOffset,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum DirectionOffset {
    #[default]
    None,
    ClockWise,
    CounterClockWise,
    Flip
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RSI {
    size: IVec2,
    path: String,
    states: HashMap<String, String>
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Vec2Yaml(pub Vec2);

impl<'de> Deserialize<'de> for Vec2Yaml {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let mut parts = s.split(',');
        //TODO это что за хуйня? какие нахуй 1, 1
        let x = parts.next().unwrap_or("1").trim().parse::<f32>().map_err(serde::de::Error::custom)?;
        let y = parts.next().unwrap_or("1").trim().parse::<f32>().map_err(serde::de::Error::custom)?;
        Ok(Vec2Yaml(Vec2::new(x, y)))
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Default)]
pub struct SpriteReplicated {
    pub rsi_path: Option<String>,
    pub state: Option<String>,
    pub color: Option<[f32; 4]>,
    pub layers: Vec<LayerReplicated>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LayerReplicated {
    pub rsi_path: Option<String>,
    pub state: Option<String>,
    pub visible: bool,
    pub color: Option<[f32; 4]>,
}

impl From<ComplexSprite> for SpriteReplicated {
    fn from(s: ComplexSprite) -> Self {
        Self {
            rsi_path: s.rsi_path.clone(),
            state: s.state.clone(),
            color: s.color.map(|c| c.to_srgba().to_f32_array()),
            layers: s.layers.iter().map(|l| LayerReplicated {
                rsi_path: l.rsi_path.clone(),
                state: l.state.clone(),
                visible: l.visible,
                color: l.color.map(|c| c.to_srgba().to_f32_array()),
            }).collect(),
        }
    }
}

impl From<SpriteReplicated> for ComplexSprite {
    fn from(r: SpriteReplicated) -> Self {
        Self {
            rsi_path: r.rsi_path,
            state: r.state,
            color: r.color.map(|[a, b, c, d]| Color::srgba(a, b, c, d)),
            layers: r.layers.into_iter().map(|l| Layer {
                rsi_path: l.rsi_path,
                state: l.state,
                visible: l.visible,
                color: l.color.map(|[a, b, c, d]| Color::srgba(a, b, c, d)),
                ..default()
            }).collect(),
            ..default()
        }
    }
}

// это не я, это нейрослоп, во всём вините его
// ох как же мне лень делать что-то самостоятельно
// надеюсь когда-нибудь это всё закончится