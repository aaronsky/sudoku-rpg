use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

pub struct PortraitViewSettings {
    pub position: [f32; 2],
    pub size: f32,
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: f32
}

impl PortraitViewSettings {
    pub fn new(x: f32, y: f32) -> Self {
        PortraitViewSettings {
            position: [x, y],
            size: 130.0,
            background_color: From::from([0.8, 0.8, 1.0, 1.0]),
            border_color: From::from([0.0, 0.0, 0.0, 1.0]),
            border_radius: 4.0
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

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::{DrawMode, Rect};

        let ref settings = self.settings;

        graphics::set_color(ctx, settings.background_color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(
                settings.position[0],
                settings.position[1],
                settings.size,
                settings.size,
            ),
        )?;

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
