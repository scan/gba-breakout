#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GameState {
    #[default]
    Start,
    Running,
    GameOver,
}
