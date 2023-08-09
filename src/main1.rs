use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, GameResult};
use crate::event::KeyCode;
//use ggez::input::keyboard::KeyCode;

struct MainState {
    // Agregar campos para representar la posición y velocidad del personaje
    position: nalgebra::Point2<f32>,
    velocity: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Configurar la posición inicial y la velocidad del personaje
        let position = nalgebra::Point2::new(400.0, 300.0);
        let velocity = 5.0;

        Ok(MainState { position, velocity })
    }
}

impl EventHandler<KeyCode> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // No es necesario actualizar la posición aquí,
        // la posición se actualiza en key_down_event
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Up => {
                // Mover hacia arriba (disminuir la coordenada Y)
                self.position.y -= self.velocity;
            }
            event::KeyCode::Down => {
                // Mover hacia abajo (aumentar la coordenada Y)
                self.position.y += self.velocity;
            }
            event::KeyCode::Left => {
                // Mover hacia la izquierda (disminuir la coordenada X)
                self.position.x -= self.velocity;
            }
            event::KeyCode::Right => {
                // Mover hacia la derecha (aumentar la coordenada X)
                self.position.x += self.velocity;
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);

        // Dibuja el personaje en su posición actual (representado aquí por un círculo)
        let character_color = graphics::Color::new(0.0, 0.0, 1.0, 1.0); // Color azul
        let character_radius = 20.0;
        let character_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            character_radius,
            0.1,
            character_color,
        )?;
        graphics::draw(ctx, &character_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pacman_game", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
