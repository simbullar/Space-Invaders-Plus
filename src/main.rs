extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::joystick::BUTTON_COUNT;
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
    game_title.set_origin(Vector2f::new(294.0, 30.0));
    game_title.set_position(Vector2f::new(WIDTH as f32 / 2.0, 260.0));
    game_title.set_scale(1.3);

    // button_play definitions
    let button_play_off_texture =
        Texture::from_file("assets/playButtonOff.png").expect("Failed to load background texture");
    let button_play_on_texture =
        Texture::from_file("assets/playButtonOn.png").expect("Failed to load background texture");
    let mut button_play = Sprite::new();
    button_play.set_texture(&button_play_off_texture, false);
    button_play.set_origin(Vector2f::new(250.0, 62.5));
    button_play.set_position(Vector2f::new(WIDTH as f32 / 2.0, 425.0));

    //buttonSettings definitions
    let settings_texture =
        Texture::from_file("assets/gear.png").expect("Failed to load background texture");
    let mut button_settings = Sprite::new();
    button_settings.set_texture(&settings_texture, false);
    button_settings.set_origin(Vector2f::new(24.0, 24.0));
    button_settings.set_position(Vector2f::new(WIDTH as f32 - 48.0, 48.0));

    // ------------------------------------ GAME DEFINITIONS ---------------------------------------
    // player definitions
    let ship_texture_default =
        Texture::from_file("assets/ship.png").expect("Failed to load background texture");
    let ship_texture_right =
        Texture::from_file("assets/rightShip.png").expect("Failed to load background texture");
    let ship_texture_left =
        Texture::from_file("assets/rightShip.png").expect("Failed to load background texture");
    let mut ship = Sprite::new();
    ship.set_texture(&ship_texture_default, false);
    ship.set_scale(0.1);

    //ball definitions
    let mut position_mouse_temporary = Vector2i::new(0, 0);
    let ball_indent = 15.0;

    // ball positions
    let ball_position_0 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 112.0);
    let ball_position_1 = Vector2f::new(
        WIDTH as f32 - 170.0 - ball_indent,
        HEIGHT as f32 - 110.0 - ball_indent,
    );
    let ball_position_2 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 105.0 - ball_indent);
    let ball_position_3 = Vector2f::new(
        WIDTH as f32 - 170.0 + ball_indent,
        HEIGHT as f32 - 110.0 - ball_indent,
    );
    let ball_position_4 = Vector2f::new(WIDTH as f32 - 170.0 - ball_indent, HEIGHT as f32 - 105.0);
    let ball_position_5 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 105.0);
    let ball_position_6 = Vector2f::new(WIDTH as f32 - 170.0 + ball_indent, HEIGHT as f32 - 105.0);
    let ball_position_7 = Vector2f::new(
        WIDTH as f32 - 170.0 - ball_indent,
        HEIGHT as f32 - 105.0 + ball_indent,
    );
    let ball_position_8 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 105.0 + ball_indent);
    let ball_position_9 = Vector2f::new(
        WIDTH as f32 - 170.0 + ball_indent,
        HEIGHT as f32 - 105.0 + ball_indent,
    );

    let difference_mouse = 40;
    let ball_texture =
        Texture::from_file("assets/ball.png").expect("Failed to load background texture");
    let mut ball = Sprite::new();
    ball.set_texture(&ball_texture, false);
    ball.set_scale(0.3);
    ball.set_position(ball_position_0);

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
    let button_on_texture =
        Texture::from_file("assets/buttonOn.png").expect("Failed to load background texture");
    let mut button = Sprite::new();
    button.set_texture(&button_off_texture, false);
    // the buttons are like 16x16 so it will be like 80x80
    button.set_scale(5.0);
    button.set_position(Vector2f::new(WIDTH as f32 - 85.0, HEIGHT as f32 - 85.0));

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
                            button_play.set_texture(&button_play_off_texture, false);
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

            let desktop_pos = mouse::desktop_position();
            let window_pos = window.position();
            let relative =
                Vector2i::new(desktop_pos.x - window_pos.x, desktop_pos.y - window_pos.y);
            let mouse_pos = window.map_pixel_to_coords(relative, &window.view());

            if button_play.global_bounds().contains(mouse_pos) {
                button_play.set_texture(&button_play_on_texture, false);
                button_play.set_scale(1.13);
            } else {
                button_play.set_texture(&button_play_off_texture, false);
                button_play.set_scale(1.0);
            }

            if button_settings.global_bounds().contains(mouse_pos) {
                button_settings.set_scale(1.7);
                button_settings.rotate(2.0);
            } else {
                button_settings.set_scale(1.5);
            }

            if game_title.global_bounds().contains(mouse_pos) {
                game_title.set_scale(1.4);
            } else {
                game_title.set_scale(1.3);
            }
            window.draw(&background);
            window.draw(&game_title);
            window.draw(&button_play);
            window.draw(&button_settings);
        } else if !is_game_over {
            // game

            // ----------- GAME LOGIC -----------
            if mouse::Button::Left.is_pressed() {
                let mouse_pos = window.mouse_position();
                if position_mouse_temporary == Vector2i::new(0, 0) {
                    position_mouse_temporary = mouse_pos;
                    ball.set_position(ball_position_5);
                } else {
                    let dx = mouse_pos.x - position_mouse_temporary.x;
                    let dy = mouse_pos.y - position_mouse_temporary.y;

                    if dx > difference_mouse {
                        if dy > difference_mouse {
                            ball.set_position(ball_position_9);
                        } else if dy < -difference_mouse {
                            ball.set_position(ball_position_3);
                        } else {
                            ball.set_position(ball_position_6);
                        }
                    } else if dx < -difference_mouse {
                        if dy > difference_mouse {
                            ball.set_position(ball_position_7);
                        } else if dy < -difference_mouse {
                            ball.set_position(ball_position_1);
                        } else {
                            ball.set_position(ball_position_4);
                        }
                    } else {
                        if dy > difference_mouse {
                            ball.set_position(ball_position_8);
                        } else if dy < -difference_mouse {
                            ball.set_position(ball_position_2);
                        } else {
                            ball.set_position(ball_position_5);
                        }
                    }
                }
            } else {
                ball.set_position(ball_position_0);
                position_mouse_temporary = Vector2i::new(0, 0);
            }

            if mouse::Button::Right.is_pressed() {
                button.set_texture(&button_on_texture, false);
            } else {
                button.set_texture(&button_off_texture, false);
            }
            // Draws for the game screen
            window.draw(&background);

            window.draw(&ship);
            window.draw(&stand);
            window.draw(&ball);
            window.draw(&button);
        } else {
            // game over
            window.draw(&background);
        }
        window.display();
    }
}
