use bevy::asset::{AssetEvent, AssetServer, Assets, Handle, LoadedFolder};
use bevy::image::Image;
use bevy::prelude::{Commands, EventReader, NextState, Res, ResMut, Resource};
use bevy::sprite::{TextureAtlasBuilder, TextureAtlasLayout};
use crate::GameState;

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load map textures
    commands.insert_resource(SpriteFolder(asset_server.load_folder("../src/assets/textures/map")));
}

fn check_textures(
    mut next_state: ResMut<NextState<GameState>>,
    sprite_folder: Res<SpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
)
{
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.0) {
            next_state.set(GameState::MainMenu); // loading assets is done
        }
    }
}

fn make_spritesheets (
    mut commands: Commands,
    sprite_handles: Res<SpriteFolder>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
)
{
    let folder = loaded_folders.get(&sprite_handles.0).unwrap();


}