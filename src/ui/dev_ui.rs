// make menu with button to spawn the dev/testing world

use bevy::prelude::{ResMut, Resource};
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, PartialEq)]
pub enum Worlds {
    Testing,
    World1,
    World2,
}

#[derive(Resource)]
pub struct DevUi {
    pub selected: Worlds,
}

impl DevUi {
    pub fn new() -> Self {
        Self {
            selected: Worlds::Testing,
        }
    }
}

pub fn dev_menu(mut contexts: EguiContexts, mut menu: ResMut<DevUi>) {
    egui::Window::new("menu").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", menu.selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut menu.selected, Worlds::Testing, "Testing");
                ui.selectable_value(&mut menu.selected, Worlds::World1, "World1");
                ui.selectable_value(&mut menu.selected, Worlds::World2, "World2");
            }
            );
    });
}