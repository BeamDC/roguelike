mod setup;
mod dev_world;
mod texture_loader;

use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{IntoSystemConfigs, OnEnter};
use crate::GameState;
use crate::world::setup::world_setup;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (world_setup).chain());
        // app.add_systems(OnEnter(GameState::TestingWorld), );
    }
}