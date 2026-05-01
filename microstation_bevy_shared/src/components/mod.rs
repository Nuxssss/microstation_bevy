use bevy::prelude::Component;
use enum_dispatch::enum_dispatch;

pub mod player;
pub mod meta_data;
pub mod transform;
pub mod sprite;
pub mod icon_smooth;
pub mod item;

pub trait ToBevyComponent {
    type Output: Component;
    fn to_bevy_component(self) -> Self::Output;
}

impl<T> ToBevyComponent for T where T: Component {
    type Output = T;
    fn to_bevy_component(self) -> T {
        self
    }
}
