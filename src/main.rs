mod ui;
mod world;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::ui::DevUiPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        EguiPlugin,
        DevUiPlugin,
    ));

    app.run();
}