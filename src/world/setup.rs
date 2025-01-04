use bevy::prelude::{Camera2d, Commands};

pub fn world_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}