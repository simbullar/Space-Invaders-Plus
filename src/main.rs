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
        &Default::default(),
    )
    .unwrap();
    window.set_vertical_sync_enabled(true);

    // ------------------------------------ MAIN MENU DEFINITIONS ------------------------------------

    // some of these are placeholders, in the future i am going to replace this with good images
    // background definitions
    let background_texture =
        Texture::from_file("assets/background.png").expect("Failed to load background texture");
    let mut background = Sprite::new();
    background.set_texture(&background_texture, false);

    // game_title definitions
    let game_title_texture =
        Texture::from_file("assets/gameTitle.png").expect("Failed to load background texture");
    let mut game_title = Sprite::new();
    game_title.set_texture(&game_title_texture, false);
    game_title.set_position(Vector2f::new(130.0, 220.0));

    // button_play definitions
    let mut button_play = RectangleShape::new();
    button_play.set_size(Vector2f::new(200.0, 50.0));
    button_play.set_fill_color(Color::rgb(255, 255, 255));
    button_play.set_position(Vector2f::new(290.0, 350.0));

    //buttonSettings definitions
    let mut button_settings = RectangleShape::new();
    button_settings.set_size(Vector2f::new(40.0, 40.0));
    button_settings.set_fill_color(Color::rgb(255, 255, 255));
    button_settings.set_position(Vector2f::new(WIDTH as f32 - 40.0, 1.0));

    // ------------------------------------ GAME DEFINITIONS ---------------------------------------
    // player definitions
    let mut player = RectangleShape::new();
    player.set_size(Vector2f::new(40.0, 40.0));

    //ball definitions
    let ball_texture =
        Texture::from_file("assets/ball.png").expect("Failed to load background texture");
    let mut ball = Sprite::new();
    ball.set_texture(&ball_texture, false);
    ball.set_scale(0.3);
    ball.set_position(Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 112.0));

    // stand definitions
    let stand_texture =
        Texture::from_file("assets/stand.png").expect("Failed to load background texture");
    let mut stand = Sprite::new();
    stand.set_texture(&stand_texture, false);
    stand.set_scale(0.7);
    stand.set_position(Vector2f::new(WIDTH as f32 - 200.0, HEIGHT as f32 - 115.0));

    // button definitions
    let button_off_texture =
        Texture::from_file("assets/buttonOff.png").expect("Failed to load background texture");
    let mut button = Sprite::new();
    button.set_texture(&button_off_texture, false);

    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                }
                Event::MouseButtonReleased { button, x, y } => {
                    if button == mouse::Button::Left && !has_game_started && !settings_opened {
                        let mouse_pos =
                            window.map_pixel_to_coords(Vector2i::new(x, y), &window.view());

                        if button_play.global_bounds().contains(mouse_pos) {
                            has_game_started = true;
                        } else if button_settings.global_bounds().contains(mouse_pos) {
                            settings_opened = true;
                        }
                    }
                }
                _ => {}
            }
        }

        // drawing
        window.clear(Color::BLACK);
        if settings_opened {
            // settings screen
            window.draw(&background);
        } else if !has_game_started {
            // menu
            window.draw(&background);
            window.draw(&game_title);
            window.draw(&button_play);
            window.draw(&button_settings);
        } else if !is_game_over {
            // game

            // ----------- GAME LOGIC -----------
            if mouse::Button::Left.is_pressed() {
                // do
            } else if mouse::Button::Right.is_pressed() {
                // change to variant
            } else {
                // change everything back
                button.set_texture(&button_off_texture, false);
            }

            // Draws for the game screen
            window.draw(&background);

            window.draw(&stand);
            window.draw(&ball)
        } else {
            // game over
            window.draw(&background);
        }
        window.display();
    }
}
