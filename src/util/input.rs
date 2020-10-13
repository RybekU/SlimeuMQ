/*!
Module for processing the raw input into easy for the game to reason about information.
Currently handles only keyboard.
*/
use enum_map::{Enum, EnumMap};
use macroquad::{is_key_down, KeyCode};

/// Treat as if the game had dedicated controller with these buttons.
#[derive(Debug, Enum, Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Up,
    Down,
    Jump,
    Attack,
}

// Reads the edge-based input and turn it into level-based.
pub struct ButtonsState {
    bindings: EnumMap<Button, (Option<KeyCode>, u8)>,
}

impl Default for ButtonsState {
    fn default() -> Self {
        let mut bindings = EnumMap::<Button, (Option<KeyCode>, u8)>::default();
        bindings[Button::Up] = (Some(KeyCode::Up), 0);
        bindings[Button::Left] = (Some(KeyCode::Left), 0);
        bindings[Button::Down] = (Some(KeyCode::Down), 0);
        bindings[Button::Right] = (Some(KeyCode::Right), 0);
        bindings[Button::Jump] = (Some(KeyCode::Space), 0);
        bindings[Button::Attack] = (Some(KeyCode::Z), 0);
        Self { bindings }
    }
}

impl ButtonsState {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn update(&mut self) {
        for (maybe_key, ref mut history) in self.bindings.values_mut() {
            if let Some(key) = maybe_key {
                *history <<= 1;
                *history |= is_key_down(*key) as u8;
            }
        }
    }
    #[allow(dead_code)]
    pub fn is_pressed(&self, button: Button) -> bool {
        (self.bindings[button].1 & 0b1) == 0b1
    }
    #[allow(dead_code)]
    pub fn pressed(&self, button: Button) -> bool {
        (self.bindings[button].1 & 0b11) == 0b01
    }
    #[allow(dead_code)]
    pub fn released(&self, button: Button) -> bool {
        (self.bindings[button].1 & 0b11) == 0b10
    }
}
