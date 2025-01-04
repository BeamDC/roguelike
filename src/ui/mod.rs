use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::Commands;
use crate::ui::dev_ui::{dev_menu, DevUi};

pub(crate) mod dev_ui;

pub struct DevUiPlugin;

fn setup(mut commands: Commands) {
    commands.insert_resource(DevUi::new());
}

impl Plugin for DevUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, dev_menu);
    }
}