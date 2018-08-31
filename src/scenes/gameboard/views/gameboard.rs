use super::super::models::Gameboard;
use ggez::graphics::{self, Color, Font, Point2, Text};
use ggez::{Context, GameResult};

pub struct GameboardViewSettings {
    pub position: [f32; 2],
    pub size: f32,
    pub background_color: Color,
    pub border_color: Color,
    pub board_edge_color: Color,
    pub section_edge_color: Color,
    pub cell_edge_color: Color,
    pub board_edge_radius: f32,
    pub section_edge_radius: f32,
    pub cell_edge_radius: f32,
    pub selected_cell_background_color: Color,
    pub text_color: Color,
    numbers: [Text; 10],
}

impl GameboardViewSettings {
    pub fn new(ctx: &mut Context) -> GameResult<GameboardViewSettings> {
        let font = Font::new(ctx, "/fonts/FiraSans/FiraSans-Regular.ttf", 34)?;
        let numbers = [
            Text::new(ctx, "0", &font)?,
            Text::new(ctx, "1", &font)?,
            Text::new(ctx, "2", &font)?,
            Text::new(ctx, "3", &font)?,
            Text::new(ctx, "4", &font)?,
            Text::new(ctx, "5", &font)?,
            Text::new(ctx, "6", &font)?,
            Text::new(ctx, "7", &font)?,
            Text::new(ctx, "8", &font)?,
            Text::new(ctx, "9", &font)?,
        ];
        Ok(GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: From::from([0.8, 0.8, 1.0, 1.0]),
            border_color: From::from([0.0, 0.0, 0.2, 1.0]),
            board_edge_color: From::from([0.0, 0.0, 0.2, 1.0]),
            section_edge_color: From::from([0.0, 0.0, 0.2, 1.0]),
            cell_edge_color: From::from([0.0, 0.0, 0.2, 1.0]),
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: From::from([0.9, 0.9, 1.0, 1.0]),
            text_color: From::from([0.0, 0.0, 0.1, 1.0]),
            numbers,
        })
    }
}

pub struct GameboardView {
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings }
    }

    pub fn draw(&self, ctx: &mut Context, gameboard: &Gameboard) -> GameResult<()> {
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

        if let Some(ind) = gameboard.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind[0] as f32 * cell_size, ind[1] as f32 * cell_size];
            graphics::set_color(ctx, settings.selected_cell_background_color)?;
            graphics::rectangle(
                ctx,
                DrawMode::Fill,
                Rect::new(
                    settings.position[0] + pos[0],
                    settings.position[1] + pos[1],
                    cell_size,
                    cell_size,
                ),
            )?;
        }

        let cell_size = settings.size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
                if let Some(ind) = gameboard.get([i, j]) {
                    let text = &settings.numbers[ind as usize];
                    let width = text.width() as f32;
                    let pos = Point2::new(
                        settings.position[0] + i as f32 * cell_size + width / 2.0,
                        settings.position[1] + j as f32 * cell_size,
                    );
                    graphics::draw_ex(
                        ctx,
                        text,
                        graphics::DrawParam {
                            dest: pos,
                            color: Some(settings.text_color),
                            ..Default::default()
                        },
                    )?;
                }
            }
        }

        let cell_edge_mesh = self.build_cell_edge_mesh(ctx, 9, 3)?;
        graphics::set_color(ctx, settings.cell_edge_color)?;
        graphics::draw_ex(ctx, &cell_edge_mesh, Default::default())?;

        let section_edge_mesh = self.build_section_edge_mesh(ctx, 3)?;
        graphics::set_color(ctx, settings.section_edge_color)?;
        graphics::draw_ex(ctx, &section_edge_mesh, Default::default())?;

        graphics::set_color(ctx, settings.board_edge_color)?;
        graphics::rectangle(
            ctx,
            DrawMode::Line(settings.board_edge_radius),
            Rect::new(
                settings.position[0],
                settings.position[1],
                settings.size,
                settings.size,
            ),
        )?;

        Ok(())
    }

    fn build_cell_edge_mesh(
        &self,
        ctx: &mut Context,
        cells: i32,
        cells_per_section: i32,
    ) -> GameResult<graphics::Mesh> {
        let ref settings = self.settings;
        let mb = graphics::MeshBuilder::new();
        for i in 0..cells {
            if (i % cells_per_section) == 0 {
                continue;
            }
            let x = settings.position[0] + i as f32 / 9.0 * settings.size;
            let y = settings.position[1] + i as f32 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = &[Point2::new(x, settings.position[1]), Point2::new(x, y2)];
            graphics::line(ctx, vline, settings.cell_edge_radius)?;

            let hline = &[Point2::new(settings.position[0], y), Point2::new(x2, y)];
            graphics::line(ctx, hline, settings.cell_edge_radius)?;
        }
        mb.build(ctx)
    }

    fn build_section_edge_mesh(
        &self,
        ctx: &mut Context,
        sections: i32,
    ) -> GameResult<graphics::Mesh> {
        let ref settings = self.settings;
        let mb = graphics::MeshBuilder::new();
        for i in 0..sections {
            let x = settings.position[0] + i as f32 / 3.0 * settings.size;
            let y = settings.position[1] + i as f32 / 3.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = &[Point2::new(x, settings.position[1]), Point2::new(x, y2)];
            graphics::line(ctx, vline, settings.section_edge_radius)?;

            let hline = &[Point2::new(settings.position[0], y), Point2::new(x2, y)];
            graphics::line(ctx, hline, settings.section_edge_radius)?;
        }
        mb.build(ctx)
    }
}
