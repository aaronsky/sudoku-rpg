use ggez;
use ggez_goodies::scene;
use specs;

use super::models::Gameboard;
use super::views::*;
use input;
use scenes::*;
use world::World;

pub struct GameboardScene {
    gameboard: Gameboard,
    view: GameboardView,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl GameboardScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let gameboard_view_settings = GameboardViewSettings::new(ctx).unwrap();
        GameboardScene {
            gameboard: Gameboard::new(),
            view: GameboardView::new(gameboard_view_settings),
            dispatcher: Self::register_systems(),
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new().build()
    }
}

impl scene::Scene<World, input::InputEvent> for GameboardScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.gameboard.solved {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.view.draw(ctx, &self.gameboard)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "Game Board"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::InputEvent, _started: bool) {
        debug!("Input: {:?}", ev);
    }
}


// impl event::EventHandler for GameboardScene {


//     fn mouse_button_up_event(
//         &mut self,
//         _ctx: &mut Context,
//         _button: MouseButton,
//         _x: i32,
//         _y: i32,
//     ) {
//         let x = self.cursor_pos[0] as f32 - self.gameboard_view.settings.position[0];
//         let y = self.cursor_pos[1] as f32 - self.gameboard_view.settings.position[1];
//         if x >= 0.0
//             && x < self.gameboard_view.settings.size
//             && y >= 0.0
//             && y < self.gameboard_view.settings.size
//         {
//             let cell_x = (x / self.gameboard_view.settings.size * 9.0) as usize;
//             let cell_y = (y / self.gameboard_view.settings.size * 9.0) as usize;
//             self.gameboard.selected_cell = Some([cell_x, cell_y]);
//         }
//     }

//     fn mouse_motion_event(
//         &mut self,
//         _ctx: &mut Context,
//         _state: MouseState,
//         x: i32,
//         y: i32,
//         _xrel: i32,
//         _yrel: i32,
//     ) {
//         self.cursor_pos = [x, y];
//     }

//     fn key_down_event(
//         &mut self,
//         _ctx: &mut Context,
//         keycode: Keycode,
//         _keymod: Mod,
//         _repeat: bool,
//     ) {
//         if let Some(ind) = self.gameboard.selected_cell {
//             match keycode {
//                 Keycode::Num1 => self.gameboard.set(ind, 1),
//                 Keycode::Num2 => self.gameboard.set(ind, 2),
//                 Keycode::Num3 => self.gameboard.set(ind, 3),
//                 Keycode::Num4 => self.gameboard.set(ind, 4),
//                 Keycode::Num5 => self.gameboard.set(ind, 5),
//                 Keycode::Num6 => self.gameboard.set(ind, 6),
//                 Keycode::Num7 => self.gameboard.set(ind, 7),
//                 Keycode::Num8 => self.gameboard.set(ind, 8),
//                 Keycode::Num9 => self.gameboard.set(ind, 9),
//                 _ => {}
//             }
//         }
//         match keycode {
//             Keycode::Left | Keycode::A => println!("left!"),
//             Keycode::Up | Keycode::W => println!("up!"),
//             Keycode::Right | Keycode::D => println!("right!"),
//             Keycode::Down | Keycode::S => println!("down!"),
//             _ => {}
//         }
//     }
// }
