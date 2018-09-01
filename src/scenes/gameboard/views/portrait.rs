use super::super::models::Character;
use ggez::graphics::{self, Color, Point2};
use ggez::{Context, GameResult};

pub struct PortraitViewSettings {
    pub position: [f32; 2],
    pub size: f32,
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: f32,
}

impl PortraitViewSettings {
    pub fn new(x: f32, y: f32) -> Self {
        PortraitViewSettings {
            position: [x, y],
            size: 130.0,
            background_color: From::from([0.8, 0.8, 1.0, 1.0]),
            border_color: From::from([0.0, 0.0, 0.0, 1.0]),
            border_radius: 4.0,
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
        use ggez::graphics::{DrawMode, Rect};

        let ref settings = self.settings;

        let pos = Point2::new(settings.position[0], settings.position[1]);
        graphics::draw(ctx, &(character.image.borrow().0), pos, 0.0)?;

        // TODO: Temporary border until we get the asset
        graphics::set_color(ctx, settings.border_color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Line(settings.border_radius),
            Rect::new(
                settings.position[0],
                settings.position[1],
                settings.size,
                settings.size,
            ),
        )?;

        Ok(())
    }
}
