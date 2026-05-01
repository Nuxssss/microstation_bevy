use bevy::color::Color;
use bevy::reflect::erased_serde::__private::serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde::de::Error;
use crate::draw_depth::DrawDepth::{LowFloors, Overlays};

#[repr(i8)]
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum DrawDepth {
    LowFloors = -20,
    ThickPipe,
    ThickWire,
    ThinPipeAlt2,
    ThinPipeAlt1,
    ThinPipe,
    ThinWire,
    BelowFloor,
    FloorTiles,
    FloorObjects,
    Puddles,
    HighFloorObjects,
    DeadMobs,
    SmallMobs,
    Walls,
    WallTops,
    #[default]
    Objects,
    SmallObjects,
    WallMountedItems,
    LargeObjects,
    Items,
    BelowMobs,
    Mobs,
    OverMobs,
    Doors,
    BlastDoors,
    Overdoors,
    Effects,
    Ghosts,
    Overlays,
}

pub(crate) fn deserialize_depth_int<'de, D: Deserializer<'de>>(d: D) -> Result<DrawDepth, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum EnumOrInt {
        Enum(DrawDepth),
        Int(i8)
    }
    match EnumOrInt::deserialize(d)? {
        EnumOrInt::Enum(a) => Ok(a),
        EnumOrInt::Int(a) => {
            if a < LowFloors as i8 || a > Overlays as i8 {
                return Err(D::Error::custom("wrong depth"));
            }
            Ok( unsafe { std::mem::transmute(a) }) // Я хуею что у них там за парсер ямла
        }
    }
}

impl DrawDepth {
    pub fn as_z(self) -> f32 {
        self as i32 as f32 * 0.01
    }
}