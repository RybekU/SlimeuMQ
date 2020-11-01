pub struct AiControlled {
    // state: PlayerState,
}

impl AiControlled {
    pub fn new() -> Self {
        Self {
            // state: PlayerState::Idle,
        }
    }
}

enum AiState {
    Idle,
    Hurt,
}
