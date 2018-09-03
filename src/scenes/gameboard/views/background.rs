use common::resources;
use ggez::graphics::{self, DrawParam, Point2};
use ggez::{Context, GameResult};
use warmy;
use world::World;

#[derive(Debug, Clone)]
pub struct BackgroundViewSettings {
    image: warmy::Res<resources::Image>,
}

impl BackgroundViewSettings {
    pub fn new(image_asset: &str, ctx: &mut Context, world: &mut World) -> GameResult<Self> {
        let image = world
            .assets
            .get::<_, resources::Image>(
                &warmy::FSKey::new(format!("/images/backgrounds/{}", image_asset)),
                ctx,
            ).unwrap();
        Ok(BackgroundViewSettings { image })
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundView {
    pub settings: BackgroundViewSettings,
}

impl BackgroundView {
    pub fn new(settings: BackgroundViewSettings) -> Self {
        BackgroundView { settings }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let settings = &self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;

        let pos: Point2 = Point2::new(0.0, 0.0);
        let scale = Point2::new(1.0, 1.0);

        graphics::draw_ex(
            ctx,
            &(settings.image.borrow().0),
            DrawParam {
                dest: pos,
                scale,
                ..Default::default()
            },
        )?;
        Ok(())
    }
}
