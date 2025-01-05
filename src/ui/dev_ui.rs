// make menu with button to spawn the dev/testing world

use crate::GameState;
use bevy::prelude::{NextState, ResMut, Resource, State};
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct DevUi {
    pub state: GameState,
}

impl DevUi {
    pub fn new() -> Self {
        Self {
            state: GameState::MainMenu,
        }
    }
}

pub fn dev_menu(
    mut contexts: EguiContexts,
    mut menu: ResMut<DevUi>,
    mut next_state: ResMut<NextState<GameState>>,
)
{
    egui::Window::new("menu").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("State Selector")
            .selected_text(format!("{:?}", menu.state))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut menu.state, GameState::MainMenu, "Main Menu");
                ui.selectable_value(&mut menu.state, GameState::Loading, "Loading");
                ui.selectable_value(&mut menu.state, GameState::TestingWorld, "Testing World");
                ui.selectable_value(&mut menu.state, GameState::World1, "World 1");
            });
    });

    // switch to the selected state
    next_state.set(menu.state.clone());
}