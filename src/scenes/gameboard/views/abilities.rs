use super::super::models::Ability;
use common::colors;
use common::resources;
use common::util::*;
use ggez::graphics::{self, Point2, Rect};
use ggez::{Context, GameResult};
use warmy;
use world::World;

const PLACEHOLDER_SIZE: f32 = 70.0;
const LEADING_PADDING: f32 = 15.0;

#[derive(Debug, Clone)]
pub struct AbilitiesViewSettings {
    pub position: Point2,
    pub size: Point2,
    background: warmy::Res<resources::Image>,
}

impl AbilitiesViewSettings {
    pub fn new(ctx: &mut Context, world: &mut World) -> Self {
        let background = world
            .assets
            .get::<_, resources::Image>(&warmy::FSKey::new("/images/ui/ability-container.png"), ctx)
            .unwrap();
        AbilitiesViewSettings {
            position: Point2::new(500.0, 215.0),
            size: Point2::new(270.0, 100.0),
            background,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AbilitiesView {
    pub settings: AbilitiesViewSettings,
}

impl AbilitiesView {
    pub fn new(settings: AbilitiesViewSettings) -> Self {
        AbilitiesView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, abilities: &[Ability]) -> GameResult<()> {
        let settings = &self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;

        let background = &(settings.background.borrow().0);
        let pos = Point2::new(settings.position.x, settings.position.y);
        graphics::draw(ctx, background, pos, 0.0)?;

        let sum_of_widths = PLACEHOLDER_SIZE * 3.0;
        let horizontal_padding =
            (background.width() as f32 - LEADING_PADDING - sum_of_widths) / 3.0;
        for (i, ability) in abilities.iter().enumerate() {
            self.draw_ability_at_index(ctx, Some(ability), i as u32, horizontal_padding)?;
        }

        if abilities.len() < 3 {
            for i in abilities.len()..3 {
                self.draw_ability_at_index(ctx, None, i as u32, horizontal_padding)?;
            }
        }

        Ok(())
    }

    fn draw_ability_at_index(
        &self,
        ctx: &mut Context,
        ability: Option<&Ability>,
        index: u32,
        horizontal_padding: f32,
    ) -> GameResult<()> {
        let settings = &self.settings;
        let get_pos = |width, height, container_height| {
            let x =
                settings.position.x + LEADING_PADDING + (horizontal_padding + width) * index as f32;
            center_rect_vertically(
                Rect::new(x, settings.position.y, 0.0, height),
                container_height,
            )
        };

        let background = &(settings.background.borrow().0);
        if let Some(ability) = ability {
            let icon = &(ability.icon.borrow().0);
            let pos = get_pos(
                icon.width() as f32,
                icon.height() as f32,
                background.height() as f32,
            );
            graphics::set_color(ctx, graphics::WHITE)?;
            graphics::draw(ctx, &(ability.icon.borrow().0), pos, 0.0)?;
        } else {
            let pos = get_pos(70.0, 70.0, background.height() as f32);
            graphics::set_color(ctx, graphics::BLACK)?;
            graphics::rectangle(
                ctx,
                graphics::DrawMode::Fill,
                Rect::new(pos.x, pos.y, 70.0, 70.0),
            )?;
        }
        Ok(())
    }
}
