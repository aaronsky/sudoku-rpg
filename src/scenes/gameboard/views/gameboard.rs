use super::super::models::{Gameboard, Point};
use common::colors;
use common::resources;
use common::util::*;
use ggez::graphics::{self, Color, Point2, Text};
use ggez::{Context, GameResult};
use warmy;
use world::World;

pub struct GameboardViewSettings {
    pub position: Point2,
    pub size: f32,
    pub section_edge_color: Color,
    pub cell_edge_color: Color,
    pub section_edge_radius: f32,
    pub cell_edge_radius: f32,
    pub selected_cell_background_color: Color,
    pub text_color: Color,
    numbers: [NumberView; 10],
    background: warmy::Res<resources::Image>,
}

struct NumberView(Text, Color);

impl NumberView {
    fn width(&self) -> u32 {
        self.0.width()
    }

    fn height(&self) -> u32 {
        self.0.height()
    }
}

impl GameboardViewSettings {
    pub fn new(
        background_image_asset: &str,
        ctx: &mut Context,
        world: &mut World,
    ) -> GameResult<Self> {
        let warmy_font = world
            .assets
            .get::<_, resources::Font>(
                &resources::FSFontKey::new("/fonts/Multicolore.ttf", 44),
                ctx,
            ).unwrap();
        let font = &(warmy_font.borrow().0);
        let numbers = [
            NumberView(Text::new(ctx, "0", &font)?, colors::BLACK),
            NumberView(Text::new(ctx, "1", &font)?, colors::RED),
            NumberView(Text::new(ctx, "2", &font)?, colors::ORANGE),
            NumberView(Text::new(ctx, "3", &font)?, colors::YELLOW),
            NumberView(Text::new(ctx, "4", &font)?, colors::LIGHT_GREEN),
            NumberView(Text::new(ctx, "5", &font)?, colors::GREEN),
            NumberView(Text::new(ctx, "6", &font)?, colors::LIGHT_BLUE),
            NumberView(Text::new(ctx, "7", &font)?, colors::BLUE),
            NumberView(Text::new(ctx, "8", &font)?, colors::INDIGO),
            NumberView(Text::new(ctx, "9", &font)?, colors::PURPLE),
        ];
        let background = world
            .assets
            .get::<_, resources::Image>(
                &warmy::FSKey::new(format!("/images/backgrounds/{}", background_image_asset)),
                ctx,
            ).unwrap();
        Ok(GameboardViewSettings {
            position: Point2::new(55.0, 100.0),
            size: 400.0,
            section_edge_color: colors::BLACK,
            cell_edge_color: colors::BLACK,
            section_edge_radius: 4.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: From::from([0.9, 0.9, 1.0, 0.5]),
            text_color: colors::BLACK,
            numbers,
            background,
        })
    }
}

pub struct GameboardView {
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    pub fn new(settings: GameboardViewSettings) -> Self {
        GameboardView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, gameboard: &Gameboard) -> GameResult<()> {
        use ggez::graphics::{DrawMode, Rect};

        let settings = &self.settings;

        graphics::set_color(ctx, graphics::WHITE)?;
        graphics::draw(
            ctx,
            &(self.settings.background.borrow().0),
            Point2::new(settings.position.x - 25.0, settings.position.y - 25.0),
            0.0,
        )?;

        self.draw_numbers_for_board(ctx, gameboard, colors::GRAY)?;

        if let Some(Point(x, y)) = gameboard.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = Point2::new(x as f32 * cell_size, y as f32 * cell_size);
            graphics::set_color(ctx, settings.selected_cell_background_color)?;
            graphics::rectangle(
                ctx,
                DrawMode::Fill,
                Rect::new(
                    settings.position.x + pos.x,
                    settings.position.y + pos.y,
                    cell_size,
                    cell_size,
                ),
            )?;
        }

        let cell_edge_mesh = self.build_cell_edge_mesh(ctx, 9, 3)?;
        graphics::set_color(ctx, settings.cell_edge_color)?;
        graphics::draw_ex(ctx, &cell_edge_mesh, Default::default())?;

        let section_edge_mesh = self.build_section_edge_mesh(ctx, 3)?;
        graphics::set_color(ctx, settings.section_edge_color)?;
        graphics::draw_ex(ctx, &section_edge_mesh, Default::default())?;

        Ok(())
    }

    fn draw_numbers_for_board(
        &self,
        ctx: &mut Context,
        board: &Gameboard,
        background: Color,
    ) -> GameResult<()> {
        use ggez::graphics::{DrawMode, Rect};
        use sudoku::Element;

        let settings = &self.settings;
        let size = settings.size / 9.0;

        for point in board.points() {
            let x = point[0];
            let y = point[1];
            let cell = Rect::new(
                settings.position.x + x as f32 * size,
                settings.position.y + y as f32 * size,
                size,
                size,
            );
            if let Some(Element(value)) = board.current[point] {
                if !board.is_mutable(Point(x, y)) {
                    graphics::set_color(ctx, background)?;
                    graphics::rectangle(ctx, DrawMode::Fill, cell)?;
                }
                let text = &settings.numbers[value as usize];
                let text_pos = center_rect_in_rect(
                    Rect::new(
                        0.0,
                        0.0,
                        text.width() as f32 / 2.0,
                        text.height() as f32 / 2.0,
                    ),
                    cell,
                );
                let text_scale = Point2::new(0.5, 0.5);
                graphics::draw_ex(
                    ctx,
                    &text.0,
                    graphics::DrawParam {
                        dest: Point2::new(text_pos.x, text_pos.y + 4.0),
                        color: Some(text.1),
                        scale: text_scale,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    fn build_cell_edge_mesh(
        &self,
        ctx: &mut Context,
        cells: i32,
        cells_per_section: i32,
    ) -> GameResult<graphics::Mesh> {
        let settings = &self.settings;
        let mut mb = graphics::MeshBuilder::new();
        for i in 0..cells {
            if (i % cells_per_section) == 0 {
                continue;
            }
            let x = settings.position.x + i as f32 / 9.0 * settings.size;
            let y = settings.position.y + i as f32 / 9.0 * settings.size;
            let x2 = settings.position.x + settings.size;
            let y2 = settings.position.y + settings.size;

            let vline = &[Point2::new(x, settings.position.y), Point2::new(x, y2)];
            mb.line(vline, settings.cell_edge_radius);

            let hline = &[Point2::new(settings.position.x, y), Point2::new(x2, y)];
            mb.line(hline, settings.cell_edge_radius);
        }
        mb.build(ctx)
    }

    fn build_section_edge_mesh(
        &self,
        ctx: &mut Context,
        sections: i32,
    ) -> GameResult<graphics::Mesh> {
        let settings = &self.settings;
        let mut mb = graphics::MeshBuilder::new();
        for i in 1..sections {
            let x = settings.position.x + i as f32 / 3.0 * settings.size;
            let y = settings.position.y + i as f32 / 3.0 * settings.size;
            let x2 = settings.position.x + settings.size;
            let y2 = settings.position.y + settings.size;

            let vline = &[Point2::new(x, settings.position.y), Point2::new(x, y2)];
            mb.line(vline, settings.section_edge_radius);

            let hline = &[Point2::new(settings.position.x, y), Point2::new(x2, y)];
            mb.line(hline, settings.section_edge_radius);
        }
        mb.build(ctx)
    }
}
