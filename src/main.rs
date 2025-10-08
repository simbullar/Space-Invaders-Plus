extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut is_game_over: bool = false;
    let mut has_game_started: bool = false;
    let mut settings_opened: bool = false;

    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Space Invaders+!!",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_vertical_sync_enabled(true);

    // all of those are placeholders, in the future i am going to replace this with good images

    // buttonPlay definitions
    let mut button_play = RectangleShape::new();
    button_play.set_size(Vector2f::new(200.0, 50.0));
    button_play.set_fill_color(Color::rgb(255, 255, 255));
    button_play.set_position(Vector2f::new(300.0, 350.0));
    let button_play_bounds = button_play.global_bounds();

    //buttonSettings definitions
    let mut button_settings = RectangleShape::new();
    button_settings.set_size(Vector2f::new(40.0, 40.0));
    button_settings.set_fill_color(Color::rgb(255, 255, 255));
    button_settings.set_position(Vector2f::new(WIDTH as f32 - 40.0, 0.0));
    let button_settings_bounds = button_settings.global_bounds();

    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                }
                Event::MouseButtonReleased { button, x, y } => {
                    if button == mouse::Button::Left {
                        if button_play_bounds.contains(Vector2f::new(x as f32, y as f32)) {
                            has_game_started = true;
                        } else if button_settings_bounds.contains(Vector2f::new(x as f32, y as f32))
                        {
                            settings_opened = true;
                        } else {
                            print!("hmm");
                        }
                    }
                }
                _ => {}
            }
        }

        // drawing
        window.clear(Color::BLACK);
        if !has_game_started {
            // menu screen
            window.draw(&button_play);
            window.draw(&button_settings);
        } else if settings_opened {
            // settings screen
        } else {
            // play again screen
            // 
        }

        window.display();
    }
}
