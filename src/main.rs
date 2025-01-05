mod ui;
mod world;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::ui::DevUiPlugin;


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    TestingWorld,
    World1,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Paused,
    Running,
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        EguiPlugin,
        DevUiPlugin,
    ));

    app.init_state::<GameState>();
    app.init_state::<PausedState>();

    app.run();
}