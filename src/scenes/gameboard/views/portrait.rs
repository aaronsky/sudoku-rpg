use super::super::models::{Character, CharacterKind};
use common::resources;
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};
use warmy;
use world::World;

pub struct PortraitViewSettings {
    pub position: [f32; 2],
    background: warmy::Res<resources::Image>,
}

impl PortraitViewSettings {
    pub fn new(kind: CharacterKind, x: f32, y: f32, ctx: &mut Context, world: &mut World) -> Self {
        let kind_asset = match kind {
            CharacterKind::Character => "character",
            CharacterKind::Opponent => "opponent",
        };
        let background = world
            .assets
            .get::<_, resources::Image>(
                &warmy::FSKey::new(format!("/images/ui/{}-portrait.png", kind_asset)),
                ctx,
            ).unwrap();
        PortraitViewSettings {
            position: [x, y],
            background,
        }
    }
}

pub struct PortraitView {
    pub settings: PortraitViewSettings,
}

impl PortraitView {
    pub fn new(settings: PortraitViewSettings) -> Self {
        PortraitView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, character: &Character) -> GameResult<()> {
        let ref settings = self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;

        let pos = Point2::new(settings.position[0], settings.position[1]);
        graphics::draw(ctx, &(settings.background.borrow().0), pos, 0.0)?;

        let pos = Point2::new(settings.position[0], settings.position[1]);
        graphics::draw(ctx, &(character.image.borrow().0), pos, 0.0)?;

        Ok(())
    }
}
