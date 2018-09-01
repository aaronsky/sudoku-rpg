use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

pub struct TimerViewSettings {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: f32
}

impl TimerViewSettings {
    pub fn new() -> Self {
        TimerViewSettings {
            position: [500.0, 435.0],
            size: [270.0, 100.0],
            background_color: From::from([0.8, 0.8, 1.0, 1.0]),
            border_color: From::from([0.0, 0.0, 0.0, 1.0]),
            border_radius: 4.0
        }
    }
}

pub struct TimerView {
    pub settings: TimerViewSettings,
}

impl TimerView {
    pub fn new(settings: TimerViewSettings) -> Self {
        TimerView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, time: u64) -> GameResult<()> {
        use ggez::graphics::{DrawMode, Rect};

        let ref settings = self.settings;

        graphics::set_color(ctx, settings.background_color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(
                settings.position[0],
                settings.position[1],
                settings.size[0],
                settings.size[1],
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
                settings.size[0],
                settings.size[1],
            ),
        )?;
        
        Ok(())
    }
}
