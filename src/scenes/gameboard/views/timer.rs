use common::resources;
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};
use warmy;
use world::World;

pub struct TimerViewSettings {
    pub position: Point2,
    background: warmy::Res<resources::Image>,
}

impl TimerViewSettings {
    pub fn new(ctx: &mut Context, world: &mut World) -> Self {
        let background = world
            .assets
            .get::<_, resources::Image>(&warmy::FSKey::new("/images/ui/timer-container.png"), ctx)
            .unwrap();
        TimerViewSettings {
            position: Point2::new(500.0, 435.0),
            background,
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
        let settings = &self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;
        graphics::draw(
            ctx,
            &(settings.background.borrow().0),
            settings.position,
            0.0,
        )?;

        Ok(())
    }
}
