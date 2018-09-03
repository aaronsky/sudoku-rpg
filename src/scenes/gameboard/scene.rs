use ggez::graphics::Point2;
use ggez::*;
use ggez_goodies::scene;
use specs;

use super::models::*;
use super::views::*;
use input;
use scenes::*;
use world::World;

pub struct GameboardScene {
    // Models
    gameboard: Gameboard,
    character: Character,
    opponent: Character,

    // Views
    background_view: BackgroundView,
    gameboard_view: GameboardView,
    abilities_view: AbilitiesView,
    timer_view: TimerView,
    character_portrait_view: PortraitView,
    opponent_portrait_view: PortraitView,

    // Component dispatcher
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl GameboardScene {
    pub fn new(ctx: &mut Context, world: &mut World) -> Self {
        use sudoku::Difficulty;
        GameboardScene {
            gameboard: Gameboard::new(Difficulty::Advanced),
            character: Character::new(
                "Main",
                CharacterKind::Character,
                "placeholder.png",
                ctx,
                world,
            ).add_ability(Ability::new(
                "Reveal Numbers",
                "placeholder.png",
                ctx,
                world,
            )),
            opponent: Character::new(
                "Opponent",
                CharacterKind::Opponent,
                "placeholder.png",
                ctx,
                world,
            ),

            background_view: BackgroundView::new(
                BackgroundViewSettings::new("area-1.png", ctx, world).unwrap(),
            ),
            gameboard_view: GameboardView::new(
                GameboardViewSettings::new("area-1-board.png", ctx, world).unwrap(),
            ),
            abilities_view: AbilitiesView::new(AbilitiesViewSettings::new(ctx, world)),
            timer_view: TimerView::new(TimerViewSettings::new(ctx, world)),
            character_portrait_view: PortraitView::new(PortraitViewSettings::new(
                CharacterKind::Character,
                Point2::new(500.0, 75.0),
                ctx,
                world,
            )),
            opponent_portrait_view: PortraitView::new(PortraitViewSettings::new(
                CharacterKind::Opponent,
                Point2::new(640.0, 75.0),
                ctx,
                world,
            )),

            dispatcher: Self::register_systems(),
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new().build()
    }
}

impl scene::Scene<World, input::InputEvent> for GameboardScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        self.dispatcher.dispatch(&gameworld.specs_world.res);
        if self.gameboard.is_solved() {
            println!("solved!");
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut World, ctx: &mut Context) -> GameResult<()> {
        self.background_view.draw(ctx)?;
        self.character_portrait_view.draw(ctx, &self.character)?;
        self.opponent_portrait_view.draw(ctx, &self.opponent)?;
        self.abilities_view.draw(ctx, &self.character.abilities)?;
        self.gameboard_view.draw(ctx, &self.gameboard)?;
        self.timer_view.draw(ctx, 0)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "Game Board"
    }

    fn input(&mut self, _gameworld: &mut World, ev: input::InputEvent, started: bool) {
        use input::{events::InputEffect, Button};

        match (ev, self.gameboard.selected_cell) {
            (InputEffect::Axis(axis, is_positive), _) if !started => {
                self.gameboard.move_selected_cell(axis, is_positive)
            }
            (InputEffect::Button(button, None), Some(point)) => match button {
                Button::Delete => {
                    if self.gameboard.is_mutable(point) {
                        let _ = self.gameboard.remove(point);
                    }
                }
                Button::Num1 => self.assign_number(point, 1),
                Button::Num2 => self.assign_number(point, 2),
                Button::Num3 => self.assign_number(point, 3),
                Button::Num4 => self.assign_number(point, 4),
                Button::Num5 => self.assign_number(point, 5),
                Button::Num6 => self.assign_number(point, 6),
                Button::Num7 => self.assign_number(point, 7),
                Button::Num8 => self.assign_number(point, 8),
                Button::Num9 => self.assign_number(point, 9),
                _ => {}
            },
            (InputEffect::Button(Button::Select, Some((x, y))), _) => self.handle_mouse(x, y),
            (_, _) => {}
        }
    }
}

impl GameboardScene {
    fn assign_number(&mut self, point: Point, num: u8) {
        use sudoku::Element;

        if self.gameboard.is_mutable(point) {
            self.gameboard.insert(point, Element(num))
        }
    }

    fn handle_mouse(&mut self, x: i32, y: i32) {
        let x = x as f32 - self.gameboard_view.settings.position.x;
        let y = y as f32 - self.gameboard_view.settings.position.y;
        self.gameboard.selected_cell = if x >= 0.0
            && x < self.gameboard_view.settings.size
            && y >= 0.0
            && y < self.gameboard_view.settings.size
        {
            let cell_x = (x / self.gameboard_view.settings.size * 9.0) as u8;
            let cell_y = (y / self.gameboard_view.settings.size * 9.0) as u8;
            Some(Point(cell_x, cell_y))
        } else {
            None
        }
    }
}
