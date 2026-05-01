use std::collections::{HashMap, HashSet};
use bevy::prelude::TypePath;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_nested_with::serde_nested;
use crate::components::meta_data::MetaData;
use crate::helpers::force_string;
use crate::prototypes::tile::TilePrototype;

#[derive(TypePath, Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Prototype {
    Entity(EntityPrototype),
    Tile(TilePrototype),
    #[serde(other)]
    Other
}

#[serde_nested]
#[serde_inline_default]
#[derive(TypePath, Debug, Deserialize, Clone, Default)]
pub struct EntityPrototype {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde_nested(sub = "String", serde(deserialize_with = "force_string"))]
    #[serde(default)]
    pub suffix: Option<String>,
    #[serde(rename = "localizationId", default)]
    pub localization_id: Option<String>,
    #[serde(default)]
    pub categories: HashSet<String>,
    //#[serde(rename = "placement", default)]
    //pub placement: Option<EntityPlacementProperties>,
    #[serde_inline_default(true)]
    pub save: bool,
    #[serde(default)]
    pub parent: Option<Parent>,
    #[serde(default)]
    pub r#abstract: bool,
    #[serde(default)]
    pub components: Vec<YamlComponent>,
    #[serde(default)]
    pub loc: HashMap<String, String>,
}

#[derive(TypePath, Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Parent {
    One(String),
    Many(Vec<String>)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum YamlComponent {
    MetaData (MetaData),
    Transform(crate::components::transform::Transform),
    Sprite (crate::components::sprite::ComplexSprite),
    IconSmooth(crate::components::icon_smooth::IconSmooth),
    Item(crate::components::item::Item),
    // Physics {
    //     //#[serde(rename = "bodyType", default)]
    //     //body_type: BodyType,
    // },
    // Fixtures {
    //     //#[serde(default)]
    //     //fixtures: HashMap<String, Fixture>,
    // },
    #[serde(other)]
    Other
}