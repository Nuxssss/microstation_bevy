use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Tile {
    /// Internal type ID (0 = Space/empty)
    pub type_id: u16,
    /// Custom flags for game logic (bitfield)
    pub flags: u8,
    /// Visual variant index (0-255)
    pub variant: u8,
    /// Rotation (0-3) + mirroring bit (4-7)
    pub rotation_mirroring: u8,
}

impl Tile {
    pub const EMPTY: Self = Self { type_id: 0, flags: 0, variant: 0, rotation_mirroring: 0 };

    #[inline]
    pub fn is_empty(self) -> bool { self.type_id == 0 }

    #[inline]
    pub fn with_variant(self, variant: u8) -> Self { Self { variant, ..self } }

    #[inline]
    pub fn with_flag(self, flag: u8) -> Self { Self { flags: self.flags | flag, ..self } }

    /// Decode rotation: 0=N, 1=E, 2=S, 3=W
    #[inline]
    pub fn rotation(self) -> u8 { self.rotation_mirroring & 0b11 }

    /// Is tile mirrored?
    #[inline]
    pub fn is_mirrored(self) -> bool { (self.rotation_mirroring & 0b100) != 0 }
}