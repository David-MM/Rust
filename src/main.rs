use ggez;
use ggez::event;
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState {
    text: Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::default();
        let text = Text::new(("¡Hola, Mundo!", font, 48.0));

        Ok(MainState { text })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        graphics::draw(ctx, &self.text, DrawParam::default().dest(na::Point2::new(100.0, 100.0)))?;
        graphics::present(ctx)
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hola_mundo_gui", "ggez").add_resource_path(".").window_setup(ggez::conf::WindowSetup::default().title("¡Hola, Mundo!")).window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
