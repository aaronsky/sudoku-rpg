//! Typedefs for input shortcuts.
use ggez::event::*;
use ggez_goodies::input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Exit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    Vert,
    Horz,
}

pub type InputBinding = input::InputBinding<Axis, Button>;
pub type InputEvent = input::InputEffect<Axis, Button>;
pub type InputState = input::InputState<Axis, Button>;

/// Create the default keybindings for our input state.
pub fn create_input_binding() -> input::InputBinding<Axis, Button> {
    input::InputBinding::new()
        .bind_key_to_axis(Keycode::Up, Axis::Vert, true)
        .bind_key_to_axis(Keycode::Down, Axis::Vert, false)
        .bind_key_to_axis(Keycode::Left, Axis::Horz, false)
        .bind_key_to_axis(Keycode::Right, Axis::Horz, true)
        .bind_key_to_button(Keycode::Num1, Button::Num1)
        .bind_key_to_button(Keycode::Num2, Button::Num2)
        .bind_key_to_button(Keycode::Num3, Button::Num3)
        .bind_key_to_button(Keycode::Num4, Button::Num4)
        .bind_key_to_button(Keycode::Num5, Button::Num5)
        .bind_key_to_button(Keycode::Num6, Button::Num6)
        .bind_key_to_button(Keycode::Num7, Button::Num7)
        .bind_key_to_button(Keycode::Num8, Button::Num8)
        .bind_key_to_button(Keycode::Num9, Button::Num9)
        .bind_key_to_button(Keycode::Escape, Button::Exit)
}
