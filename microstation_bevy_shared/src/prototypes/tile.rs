use serde_inline_default::serde_inline_default;
use bevy::prelude::*;
use serde::Deserialize;

// В конец файла, после YamlComponent
#[serde_inline_default]
#[derive(TypePath, Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TilePrototype {
    #[serde(deserialize_with = "crate::helpers::force_string")]
    pub id: String,

    #[serde(default)]
    pub parent: Option<String>,

    // ✅ Рендер (PNG-спрайт)
    #[serde(default)]
    pub sprite: Option<String>,

    // ✅ Физика движения (замедление)
    #[serde_inline_default(1.0)]
    pub friction: f32,

    // ✅ Коллизии (проходим ли тайл)
    #[serde_inline_default(false)]
    pub is_space: bool,

    // ✅ Рендер вариантов
    #[serde_inline_default(1u8)]
    pub variants: u8,

    // TODO

    // #[serde(default)]
    // pub name: Option<String>, // UI / Локализация

    // #[serde(default)]
    // pub r#abstract: bool, // Валидатор прототипов

    // #[serde(default)]
    // pub is_subfloor: bool, // Система слоёв/подложки

    // #[serde_inline_default(10000)]
    // pub heat_capacity: u32, // Атмосферика / Термодинамика

    // #[serde(default)]
    // pub footstep_sounds: Option<FootstepSound>, // Аудиоподсистема

    // #[serde(default)]
    // pub base_turf: Option<String>, // Система генерации карт / турфов

    // #[serde(default)]
    // pub base_whitelist: Vec<String>, // Валидация строительства

    // #[serde(default)]
    // pub indestructible: bool, // Деконструкция / Урон

    // #[serde(default)]
    // pub item_drop: Option<String>, // Деконструкция / Дроп

    // #[serde(default)]
    // pub placement_variants: Vec<f32>, // Рандомизатор при постройке

    // #[serde(default)]
    // pub deconstruct_tools: Vec<String>, // Система инструментов

    // #[serde(default)]
    // pub weather: bool, // Погода / Влияние среды

    // #[serde(default)]
    // pub mass: u32, // Физика тел / Инерция
}

// #[derive(Debug, Clone, Deserialize, Default)]
// pub struct FootstepSound {
//     pub collection: String,
// }