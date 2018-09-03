use super::super::models::{Character, CharacterKind};
use common::resources;
use common::util::*;
use ggez::graphics::{self, Point2, Rect};
use ggez::{Context, GameResult};
use warmy;
use world::World;

#[derive(Debug, Clone)]
pub struct PortraitViewSettings {
    pub position: Point2,
    background: warmy::Res<resources::Image>,
}

impl PortraitViewSettings {
    pub fn new(
        kind: CharacterKind,
        position: Point2,
        ctx: &mut Context,
        world: &mut World,
    ) -> Self {
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
            position,
            background,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PortraitView {
    pub settings: PortraitViewSettings,
}

impl PortraitView {
    pub fn new(settings: PortraitViewSettings) -> Self {
        PortraitView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, character: &Character) -> GameResult<()> {
        let settings = &self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;

        let background_image = &(settings.background.borrow().0);
        let pos = Point2::new(settings.position[0], settings.position[1]);
        graphics::draw(ctx, background_image, pos, 0.0)?;

        let character_image = &(character.image.borrow().0);
        let pos = center_rect_in_rect(
            Rect::new(
                0.0,
                0.0,
                character_image.width() as f32,
                character_image.height() as f32,
            ),
            Rect::new(
                settings.position.x,
                settings.position.y,
                background_image.width() as f32,
                background_image.height() as f32,
            ),
        );
        graphics::draw(ctx, character_image, pos, 0.0)?;

        Ok(())
    }
}
