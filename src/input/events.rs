use ggez::event::*;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum InputType {
    KeyEvent(Keycode),
    MouseButtonEvent(MouseButton),
    MouseMotionEvent,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEffect<Axes, Buttons>
where
    Axes: Eq + Hash + Clone,
    Buttons: Eq + Hash + Clone,
{
    Axis(Axes, bool),
    Button(Buttons, Option<(i32, i32)>),
    MouseMotion(i32, i32, i32, i32),
}

#[derive(Debug, Copy, Clone)]
struct AxisState {
    // Where the axis currently is, in [-1, 1]
    position: f32,
    // Where the axis is moving towards.  Possible
    // values are -1, 0, +1
    // (or a continuous range for analog devices I guess)
    direction: f32,
    // Speed in units per second that the axis
    // moves towards the target value.
    acceleration: f32,
    // Speed in units per second that the axis will
    // fall back toward 0 if the input stops.
    gravity: f32,
}

impl Default for AxisState {
    fn default() -> Self {
        AxisState {
            position: 0.0,
            direction: 0.0,
            acceleration: 4.0,
            gravity: 3.0,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct ButtonState {
    pressed: bool,
    pressed_last_frame: bool,
}
/// A struct that contains a mapping from physical input events
/// (currently just `Keycode`s) to whatever your logical Axis/Button
/// types are.
pub struct InputBinding<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    // Once EnumSet is stable it should be used for these
    // instead of BTreeMap. ♥?
    // Binding of keys to input values.
    bindings: HashMap<InputType, InputEffect<Axes, Buttons>>,
}

impl<Axes, Buttons> InputBinding<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        InputBinding {
            bindings: HashMap::new(),
        }
    }

    /// Adds a key binding connecting the given keycode to the given
    /// logical axis.
    pub fn bind_key_to_axis(mut self, keycode: Keycode, axis: Axes, positive: bool) -> Self {
        self.bindings.insert(
            InputType::KeyEvent(keycode),
            InputEffect::Axis(axis.clone(), positive),
        );
        self
    }

    /// Adds a key binding connecting the given keycode to the given
    /// logical button.
    pub fn bind_key_to_button(mut self, keycode: Keycode, button: Buttons) -> Self {
        self.bindings.insert(
            InputType::KeyEvent(keycode),
            InputEffect::Button(button.clone(), None),
        );
        self
    }

    pub fn bind_mouse_to_button(mut self, mouse: MouseButton, button: Buttons) -> Self {
        self.bindings.insert(
            InputType::MouseButtonEvent(mouse),
            InputEffect::Button(button.clone(), None),
        );
        self
    }

    pub fn bind_mouse_motion(mut self) -> Self {
        self.bindings.insert(
            InputType::MouseMotionEvent,
            InputEffect::MouseMotion(0, 0, 0, 0),
        );
        self
    }

    /// Takes an physical input type and turns it into a logical input type (keycode -> axis/button).
    pub fn resolve_key(&self, keycode: Keycode) -> Option<InputEffect<Axes, Buttons>> {
        self.bindings.get(&InputType::KeyEvent(keycode)).cloned()
    }

    pub fn resolve_mouse(
        &self,
        mouse: MouseButton,
        x: i32,
        y: i32,
    ) -> Option<InputEffect<Axes, Buttons>> {
        if let Some(InputEffect::Button(button, _)) =
            self.bindings.get(&InputType::MouseButtonEvent(mouse))
        {
            Some(InputEffect::Button(button.clone(), Some((x, y))))
        } else {
            None
        }
    }

    pub fn resolve_mouse_motion(
        &self,
        // mouse: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) -> Option<InputEffect<Axes, Buttons>> {
        if self.bindings.contains_key(&InputType::MouseMotionEvent) {
            Some(InputEffect::MouseMotion(x, y, xrel, yrel))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct InputState<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    // Input state for axes
    axes: HashMap<Axes, AxisState>,
    // Input states for buttons
    buttons: HashMap<Buttons, ButtonState>,
    motion: i32,
}

impl<Axes, Buttons> InputState<Axes, Buttons>
where
    Axes: Eq + Hash + Clone,
    Buttons: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        InputState {
            axes: HashMap::new(),
            buttons: HashMap::new(),
            motion: 0,
        }
    }
}
