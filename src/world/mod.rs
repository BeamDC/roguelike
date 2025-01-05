mod setup;
mod dev_world;
mod world_loader;

use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoSystemConfigs;
use crate::world::setup::world_setup;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (world_setup).chain());
    }
}