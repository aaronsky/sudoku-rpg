use super::super::models::Ability;
use ggez::graphics::{self, Color, Point2};
use ggez::{Context, GameResult};

pub struct AbilitiesViewSettings {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: f32,
}

impl AbilitiesViewSettings {
    pub fn new() -> Self {
        AbilitiesViewSettings {
            position: [500.0, 215.0],
            size: [270.0, 100.0],
            background_color: From::from([0.8, 0.8, 1.0, 1.0]),
            border_color: From::from([0.0, 0.0, 0.0, 1.0]),
            border_radius: 4.0,
        }
    }
}

pub struct AbilitiesView {
    pub settings: AbilitiesViewSettings,
}

impl AbilitiesView {
    pub fn new(settings: AbilitiesViewSettings) -> Self {
        AbilitiesView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, abilities: &[Ability]) -> GameResult<()> {
        use ggez::graphics::{DrawMode, Rect};

        let ref settings = self.settings;

        for ability in abilities {
            let pos = Point2::new(settings.position[0], settings.position[1]);
            graphics::draw(ctx, &(ability.icon.borrow().0), pos, 0.0)?;
        }

        // TODO: Temporary border until we get the asset
        graphics::set_color(ctx, settings.border_color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Line(settings.border_radius),
            Rect::new(
                settings.position[0],
                settings.position[1],
                settings.size[0],
                settings.size[1],
            ),
        )?;

        Ok(())
    }
}
