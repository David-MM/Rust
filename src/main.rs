extern crate piston_window;
use piston_window::*;

struct Paddle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: [f32; 4],
}

struct Ball {
    x: f64,
    y: f64,
    radius: f64,
    x_speed: f64,
    y_speed: f64,
    color: [f32; 4],
}

impl Ball {
    fn update(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;
    }

    fn check_collision(&mut self, paddles: &[&Paddle]) {
        if self.y - self.radius < 0.0 || self.y + self.radius > 600.0 {
            self.y_speed = -self.y_speed;
        }

        for paddle in paddles {
            if self.x - self.radius < paddle.x + paddle.width
                && self.x + self.radius > paddle.x
                && self.y - self.radius < paddle.y + paddle.height
                && self.y + self.radius > paddle.y
            {
                self.x_speed = -self.x_speed;
            }
        }
    }

    fn reset(&mut self) {
        self.x = 400.0;
        self.y = 300.0;
        self.x_speed = 0.8;
        self.y_speed = 0.8;
    }
}

struct Game {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    score1: u32,
    score2: u32,
    game_over: bool,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Pong Game", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let mut game = Game {
        player1: Paddle {
            x: 20.0,
            y: 250.0,
            width: 15.0,
            height: 100.0,
            color: [1.0, 0.0, 0.0, 1.0],
        },
        player2: Paddle {
            x: 765.0,
            y: 250.0,
            width: 15.0,
            height: 100.0,
            color: [0.0, 0.0, 1.0, 1.0],
        },
        ball: Ball {
            x: 400.0,
            y: 300.0,
            radius: 10.0,
            x_speed: 0.4,
            y_speed: 0.4,
            color: [0.0, 1.0, 0.0, 1.0],
        },
        score1: 0,
        score2: 0,
        game_over: false,
    };

    while let Some(event) = window.next() {
        if !game.game_over {
            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::Up) => game.player1.y -= 25.0,
                    Button::Keyboard(Key::Down) => game.player1.y += 25.0,
                    _ => {}
                }
            }

            if let Some(button) = event.release_args() {
                match button {
                    Button::Keyboard(Key::W) => game.player2.y -= 25.0,
                    Button::Keyboard(Key::S) => game.player2.y += 25.0,
                    _ => {}
                }
            }

            game.ball.update();
            game.ball.check_collision(&[&game.player1, &game.player2]);

            if game.ball.x - game.ball.radius < 0.0 {
                game.score2 += 1;
                game.ball.reset();
            } else if game.ball.x + game.ball.radius > 800.0 {
                game.score1 += 1;
                game.ball.reset();
            }

            window.draw_2d(&event, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                // Dibuja el marcador y el nombre del jugador 1
                for i in 0..game.score1 {
                    let x = 50.0 + (i as f64) * 20.0;
                    rectangle([1.0, 1.0, 1.0, 1.0], [x, 10.0, 15.0, 20.0], c.transform, g);
                }
                text::Text::new_color([1.0, 0.0, 0.0, 1.0], 20)
                    .draw(
                        "Player 1",
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(10.0, 25.0),
                        g,
                    )
                    .unwrap();
            
                // Dibuja el marcador y el nombre del jugador 2
                for i in 0..game.score2 {
                    let x = 715.0 - (i as f64) * 20.0;
                    rectangle([1.0, 1.0, 1.0, 1.0], [x, 10.0, 15.0, 20.0], c.transform, g);
                }
                text::Text::new_color([0.0, 0.0, 1.0, 1.0], 20)
                    .draw(
                        "Player 2",
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(765.0, 25.0),
                        g,
                    )
                    .unwrap();

                rectangle(game.player1.color, [game.player1.x, game.player1.y, game.player1.width, game.player1.height], c.transform, g);
                rectangle(game.player2.color, [game.player2.x, game.player2.y, game.player2.width, game.player2.height], c.transform, g);

                ellipse(game.ball.color, [game.ball.x - game.ball.radius, game.ball.y - game.ball.radius, game.ball.radius * 2.0, game.ball.radius * 2.0], c.transform, g);
                
                if game.score1 >= 5 || game.score2 >= 5 {
                    game.game_over = true;
                }
            });
        } else {
            window.draw_2d(&event, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
    
                // Dibujar una elipse o un rectángulo grande en el centro
                let winner_color = if game.score1 >= 5 {
                    [1.0, 0.0, 0.0, 1.0] // Color para el jugador 1
                } else {
                    [0.0, 0.0, 1.0, 1.0] // Color para el jugador 2
                };
                
                // Dibuja la elipse o el rectángulo en el centro de la pantalla
                // Puedes ajustar las coordenadas, tamaños y colores según tus preferencias
                ellipse(winner_color, [300.0, 250.0, 200.0, 150.0], c.transform, g);
            });
        }
    }
}
