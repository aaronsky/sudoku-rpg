extern crate chrono;
#[macro_use]
extern crate failure;
extern crate fern;
extern crate ggez;
extern crate ggez_goodies;
#[macro_use]
extern crate log;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate warmy;

mod common;
mod components;
mod scenes;
mod systems;
mod world;

use self::common::input;

use ggez::conf;
use ggez::event;
use ggez::*;

use ggez::event::*;
use ggez::graphics;
use ggez::timer;

use std::path;

pub struct MainState {
    scenes: scenes::FSceneStack,
    input_binding: input::InputBinding,
}

impl MainState {
    pub fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> Self {
        let world = world::World::new(ctx, resource_dir.clone());
        let mut scenestack = scenes::FSceneStack::new(ctx, world);
        let initial_scene = Box::new(scenes::GameboardScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);
        MainState {
            scenes: scenestack,
            input_binding: common::create_input_binding(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update();
        }
        self.scenes.world.assets.sync(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.scenes.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, false);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: i32,
        _y: i32,
    ) {
        // TODO
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: i32,
        _y: i32,
    ) {
        // TODO
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        _x: i32,
        _y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
    }
}

pub fn main() {
    common::setup_logger().expect("Could not set up logging!");
    let mut cb = ContextBuilder::new("sudoku", "sudoku")
        .window_setup(conf::WindowSetup::default().title("Sudoku RPG"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    // And save it so we can feed there result into warmy
    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    });
    // If we have such a path then add it to the context builder too
    // (modifying the cb from inside a closure gets sticky)
    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
    }

    let ctx = &mut cb.build().unwrap();

    let state = &mut MainState::new(cargo_path, ctx);
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

// extern crate ggez;

// mod models;
// mod scenes;
// mod views;

// use ggez::conf;
// use ggez::event;
// use ggez::Context;
// use std::env;
// use std::path;

// use scenes::GameboardScene;

// fn main() {
//     let config = conf::Conf::new();
//     let ctx = &mut Context::load_from_conf("sudoku", "sky", config).unwrap();

//     if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
//         let mut path = path::PathBuf::from(manifest_dir);
//         path.push("assets");
//         ctx.filesystem.mount(&path, true);
//     }

//     println!("Full filesystem info: {:#?}", ctx.filesystem);

//     println!("Resource stats:");
//     ctx.print_resource_stats();

//     let main = &mut GameboardScene::new(ctx).expect("Could not create main scene");
//     if let Err(e) = event::run(ctx, main) {
//         println!("Error encountered: {}", e);
//     } else {
//         println!("Game exited cleanly.");
//     }
// }
