use common::resources;
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};
use warmy;
use world::World;

pub struct TimerViewSettings {
    pub position: [f32; 2],
    pub size: [f32; 2],
    background: warmy::Res<resources::Image>,
}

impl TimerViewSettings {
    pub fn new(ctx: &mut Context, world: &mut World) -> Self {
        let background = world
            .assets
            .get::<_, resources::Image>(&warmy::FSKey::new("/images/ui/timer-container.png"), ctx)
            .unwrap();
        TimerViewSettings {
            position: [500.0, 435.0],
            size: [270.0, 100.0],
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
        let ref settings = self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;

        let pos = Point2::new(settings.position[0], settings.position[1]);
        graphics::draw(ctx, &(settings.background.borrow().0), pos, 0.0)?;

        Ok(())
    }
}
