use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//TODO когда появится возможность двигать ентити, нужно будет использовать это событие
// и добить логику occupancy, чтобы менять occupancy при движении ентити
#[derive(Event, Serialize, Deserialize, Clone, Copy)]
pub struct EntityMove(pub IVec2);
