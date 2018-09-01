pub mod gameboard;

use ggez::event::{MouseState, MouseButton};
use ggez_goodies::scene;

use input;
use world::World;


pub use self::gameboard::*;

// Shortcuts for our scene type.
pub type FSceneSwitch = scene::SceneSwitch<World, input::InputEvent>;
pub type FSceneStack = scene::SceneStack<World, input::InputEvent>;

impl input::MouseEventHandler for FSceneStack {
    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        // self.current().mouse_button_down_event(&mut self.world, button, x, y);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        // self.current().mouse_button_up_event(&mut self.world, button, x, y);
    }

    fn mouse_motion_event(&mut self, state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        // self.current().mouse_motion_event(&mut self.world, state, x, y, xrel, yrel);
    }
}
