use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}
