extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::joystick::BUTTON_COUNT;
use sfml::window::*;

use sfml::system::Clock;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const BOUNDARY_TOP: f32 = 200.0;
const BOUNDARY_BOTTOM: f32 = 550.0;
const BOUNDARY_RIGHT: f32 = 350.0;
const BOUNDARY_LEFT: f32 = 50.0;
const BULLETS_COUNT: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ShipFacing {
    Neutral,
    Left,
    Right,
}
#[derive(Clone, Copy)]
enum EnemyType {
    Default,
    Armored,
    Fast,
}
struct Projectile<'a> {
    sprite: Sprite<'a>,
    speed: f32,
    direction: i32, // -1 = up, 1 = down
}

// ENEMY AND WAVES
struct Enemy<'a> {
    sprite: Sprite<'a>,
    spawn_point: Vector2f,
    enemy_type: EnemyType,
    alive: bool,
    speed: f32,
}
#[derive(Clone)]
enum EnemyPositions {
    A1,
    B1,
    C1,
    D1,
    A2,
    B2,
    C2,
    A3,
    B3,
    A4,
}

impl EnemyPositions {
    fn return_in_order() -> &'static [EnemyPositions] {
        use EnemyPositions::*;
        static POSITIONS: [EnemyPositions; 10] = [A1, B1, C1, D1, A2, B2, C2, A3, B3, A4];
        &POSITIONS
    }
    fn value(&self) -> Vector2f {
        match *self {
            EnemyPositions::A1 => Vector2f::new(WIDTH as f32 - 65.0, 1.0),
            EnemyPositions::B1 => Vector2f::new(WIDTH as f32 - 135.0, 1.0),
            EnemyPositions::C1 => Vector2f::new(WIDTH as f32 - 205.0, 1.0),
            EnemyPositions::D1 => Vector2f::new(WIDTH as f32 - 275.0, 1.0),
            EnemyPositions::A2 => Vector2f::new(WIDTH as f32 - 65.0, 71.0),
            EnemyPositions::B2 => Vector2f::new(WIDTH as f32 - 135.0, 71.0),
            EnemyPositions::C2 => Vector2f::new(WIDTH as f32 - 205.0, 71.0),
            EnemyPositions::A3 => Vector2f::new(WIDTH as f32 - 65.0, 141.0),
            EnemyPositions::B3 => Vector2f::new(WIDTH as f32 - 135.0, 141.0),
            EnemyPositions::A4 => Vector2f::new(WIDTH as f32 - 65.0, 211.0),
        }
    }
}

struct Wave<'a> {
    enemies: Vec<Enemy<'a>>,
}

use rand::seq::SliceRandom;

fn spawn_wave<'a>(texture: &'a Texture, wave_number: u32) -> Wave<'a> {
    let mut positions: Vec<EnemyPositions> = EnemyPositions::return_in_order().to_vec();
    let mut rng = rand::thread_rng();
    positions.shuffle(&mut rng);

    // Number of enemies per wave increases with wave number
    let enemy_count = (3 + wave_number as usize).min(positions.len());

    let mut enemies = Vec::new();

    for pos_enum in positions.iter().take(enemy_count) {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, false);
        sprite.set_scale(2.0);
        sprite.set_position(pos_enum.value());
        let pos = sprite.position();

        enemies.push(Enemy {
            sprite,
            speed: 0.5 + wave_number as f32 * 0.1, // gradually faster
            spawn_point: pos,
            alive: true,
            enemy_type: EnemyType::Default,
        });
    }

    Wave { enemies }
}

fn shoot<'a>(
    texture: &'a Texture,
    start_pos: Vector2f,
    speed: f32,
    direction: i32,
) -> Projectile<'a> {
    let mut projectile = Sprite::new();
    projectile.set_texture(texture, false);
    projectile.set_scale(1.0);
    projectile.set_rotation(-30.0);
    projectile.set_position(start_pos);

    Projectile {
        sprite: projectile,
        speed,
        direction,
    }
}

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
    let ship_texture_left =
        Texture::from_file("assets/leftShip.png").expect("Failed to load left ship texture");
    let ship_texture_right =
        Texture::from_file("assets/rightShip.png").expect("Failed to load right ship texture");

    let mut ship = Sprite::new();
    ship.set_texture(&ship_texture_default, false);
    ship.set_scale(2.6);
    let move_speed = 2.5;

    //ball definitions
    let ball_indent = 15.0;

    // ball positions
    let ball_position_0 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 112.0);
    let ball_position_2 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 105.0 - ball_indent);
    let ball_position_4 = Vector2f::new(WIDTH as f32 - 170.0 - ball_indent, HEIGHT as f32 - 105.0);
    let ball_position_5 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 105.0);
    let ball_position_6 = Vector2f::new(WIDTH as f32 - 170.0 + ball_indent, HEIGHT as f32 - 105.0);
    let ball_position_8 = Vector2f::new(WIDTH as f32 - 170.0, HEIGHT as f32 - 105.0 + ball_indent);
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

    let projectile_texture =
        Texture::from_file("assets/projectile.png").expect("Failed to load projectile texture");
    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut temp_shot: bool = false;
    let mut bullets_availiable = BUTTON_COUNT;
    let mut reload_clock = Clock::start().expect("RESULT");
    let reload_interval = 1.0;

    let enemy_texture =
        Texture::from_file("assets/enemyDefault.png").expect("Failed to load projectile texture");
    let mut enemies: Vec<Enemy> = Vec::new();

    let mut wave_number = 0;
    let mut current_wave = spawn_wave(&enemy_texture, wave_number);

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

            let elapsed = reload_clock.elapsed_time().as_seconds();
            if elapsed >= reload_interval {
                if bullets_availiable < 5 {
                    bullets_availiable += 1;
                }
                reload_clock.restart();
            }
            let mut ship_pos = ship.position();
            let mut scale = ship.get_scale();
            let mut moved = false;

            if Key::D.is_pressed() {
                ship_pos.x += move_speed;
                ship_pos.y += move_speed;
                ship.set_texture(&ship_texture_right, true);
                scale.x = 2.8;
                scale.y = 2.8;
                ball.set_position(ball_position_6);
                moved = true;
            }

            if Key::A.is_pressed() {
                ship_pos.x -= move_speed;
                ship_pos.y -= move_speed;
                ship.set_texture(&ship_texture_left, true);
                scale.x = 2.4;
                scale.y = 2.4;
                ball.set_position(ball_position_4);
                moved = true;
            }

            if Key::W.is_pressed() {
                ship_pos.x += move_speed;
                ship_pos.y -= move_speed;
                ship.set_texture(&ship_texture_default, true);
                scale.x = 2.7;
                scale.y = 2.7;
                moved = true;
                ball.set_position(ball_position_2);
            }

            if Key::S.is_pressed() {
                ship_pos.x -= move_speed;
                ship_pos.y += move_speed;
                ship.set_texture(&ship_texture_default, true);
                scale.x = 2.5;
                scale.y = 2.5;
                moved = true;
                ball.set_position(ball_position_8);
            }
            if !moved {
                ship.set_texture(&ship_texture_default, true);
                scale.x = 2.6;
                scale.y = 2.6;
                ball.set_position(ball_position_5);
            }

            if ship_pos.x < BOUNDARY_LEFT {
                ship_pos.x = BOUNDARY_LEFT;
            }
            if ship_pos.x > BOUNDARY_RIGHT {
                ship_pos.x = BOUNDARY_RIGHT;
            }
            if ship_pos.y < BOUNDARY_TOP {
                ship_pos.y = BOUNDARY_TOP;
            }
            if ship_pos.y > BOUNDARY_BOTTOM {
                ship_pos.y = BOUNDARY_BOTTOM;
            }

            ship.set_position(ship_pos);
            ship.set_scale(scale.x);

            if mouse::Button::Right.is_pressed() {
                button.set_texture(&button_on_texture, false);
                if !temp_shot && bullets_availiable > 0 {
                    let ship_pos = ship.position();
                    let projectile_start = Vector2f::new(ship_pos.x + 85.0, ship_pos.y + 0.0);
                    let direction_code = 1;
                    projectiles.push(shoot(
                        &projectile_texture,
                        projectile_start,
                        8.0,
                        direction_code,
                    ));
                    temp_shot = true;
                    bullets_availiable -= 1;
                }
            } else {
                button.set_texture(&button_off_texture, false);
                temp_shot = false;
            }

            window.draw(&background);

            window.draw(&ship);
            window.draw(&stand);
            window.draw(&ball);
            window.draw(&button);

            for enemy in &mut current_wave.enemies {
                let mut pos = enemy.sprite.position();
                pos.y += enemy.speed;
                pos.x -= enemy.speed;
                enemy.sprite.set_position(pos);
            }

            current_wave
                .enemies
                .retain(|e| e.sprite.position().y < HEIGHT as f32 && e.alive == true);

            if current_wave.enemies.is_empty() {
                wave_number += 1;
                current_wave = spawn_wave(&enemy_texture, wave_number);
            }

            for enemy in &current_wave.enemies {
                window.draw(&enemy.sprite);
            }
            for projectile in &mut projectiles {
                let mut pos = projectile.sprite.position();
                pos.x += projectile.speed;
                pos.y -= projectile.speed * projectile.direction as f32;
                projectile.sprite.set_position(pos);
                let proj_bounds: FloatRect = projectile.sprite.global_bounds();
                for enemy in &mut current_wave.enemies {
                    let enemy_bounds: FloatRect = enemy.sprite.global_bounds();

                    if proj_bounds.intersection(&enemy_bounds) != None {
                        enemy.alive = false;
                    }
                }
            }

            projectiles
                .retain(|p| p.sprite.position().x > 0.0 && p.sprite.position().x < WIDTH as f32);

            for projectile in &projectiles {
                window.draw(&projectile.sprite);
            }
        } else {
            // game over
            window.draw(&background);
        }
        window.display();
    }
}
