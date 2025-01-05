use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    LoadingScreen,
    MainMenu,
    TestingWorld,
    World1,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PausedState {
    #[default]
    Paused,
    Running,
}