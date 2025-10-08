extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut isGameOver: bool = false;
    let mut hasGameStarted: bool = false;

    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Space Invaders+!!",
        Style::CLOSE,
        &Default::default(),
    )
    .unwrap();
    window.set_vertical_sync_enabled(true);
    
    // all of those are placeholders, in the future i am going to replace this with good images
    
    // buttonPlay definitions
    let mut buttonPlay = RectangleShape::new();
    buttonPlay.set_size(Vector2f::new(
        WIDTH as f32 / 2.0 - 60.0,
        HEIGHT as f32 / 2.0 - 25.0,
    ));
    buttonPlay.set_fill_color(Color::rgb(255, 255, 255));
    buttonPlay.set_position(Vector2f::new(120.0, 50.0));
    

    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                }
                _ => {}
            }
        }

        // drawing
        window.clear(Color::BLACK);
        window.draw(&buttonPlay);
        window.display();
    }
}
