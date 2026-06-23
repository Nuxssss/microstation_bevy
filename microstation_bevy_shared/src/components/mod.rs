use bevy::prelude::Component;
use enum_dispatch::enum_dispatch;

pub mod icon_smooth;
pub mod item;
pub mod meta_data;
pub mod player;
pub mod sprite;
pub mod transform;

pub trait ToBevyComponent {
    type Output: Component;
    fn to_bevy_component(self) -> Self::Output;
}

impl<T> ToBevyComponent for T
where
    T: Component,
{
    type Output = T;
    fn to_bevy_component(self) -> T {
        self
    }
}
