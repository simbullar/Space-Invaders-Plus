extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::joystick::BUTTON_COUNT;
use sfml::window::*;

use rand::Rng;
use rand::seq::SliceRandom;
use sfml::system::Clock;
use std::collections::HashMap;

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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum EnemyType {
    Default,
    Armored,
    Fast,
}

impl EnemyType {
    fn default_speed(&self) -> f32 {
        match *self {
            EnemyType::Default => 1.0,
            EnemyType::Armored => 0.5,
            EnemyType::Fast => 2.0,
        }
    }
    fn default_health(&self) -> i32 {
        match *self {
            EnemyType::Default => 2,
            EnemyType::Armored => 4,
            EnemyType::Fast => 1,
        }
    }
    fn points_gained(&self) -> i32 {
        match *self {
            EnemyType::Default => 200,
            EnemyType::Armored => 400,
            EnemyType::Fast => 150,
        }
    }
}

struct Projectile<'a> {
    sprite: Sprite<'a>,
    speed: f32,
    direction: i32, // -1 = up, 1 = down
    damage: i32,
}

// ENEMY AND WAVES
struct Enemy<'a> {
    sprite: Sprite<'a>,
    spawn_point: Vector2f,
    enemy_type: EnemyType,
    alive: bool,
    speed: f32,
    health: i32,
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

fn spawn_wave<'a>(textures: &HashMap<EnemyType, &'a Texture>, wave_number: u32) -> Wave<'a> {
    let mut rng = rand::thread_rng();
    let positions = EnemyPositions::return_in_order();
    let mut enemies = Vec::new();

    // Spawn more enemies as wave_number increases
    let enemy_count = (3 + wave_number as usize).min(positions.len());

    for i in 0..enemy_count {
        // Randomly pick a variant — more fast/tank as wave number increases
        let variant = if rng.gen_bool(0.2 + 0.1 * wave_number as f64) {
            EnemyType::Fast
        } else if rng.gen_bool(0.1 + 0.05 * wave_number as f64) {
            EnemyType::Armored
        } else {
            EnemyType::Default
        };

        let texture = textures[&variant];
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, false);
        sprite.set_position(positions[i].value());
        sprite.set_scale(2.0);

        enemies.push(Enemy {
            sprite,
            speed: variant.default_speed(),
            health: variant.default_health(),
            enemy_type: variant,
            alive: true,
            spawn_point: positions[i].value(),
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
        damage: 0,
    }
}

fn main() {
    let mut is_game_over: bool = false;
    let mut has_game_started: bool = false;
    let mut settings_opened: bool = false;

    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Space Invaders+",
        Style::CLOSE,
        &Default::default(),
    )
    .unwrap();
    window.set_vertical_sync_enabled(true);

    // ------------------------------------ MAIN MENU DEFINITIONS ------------------------------------

    let background_texture =
        Texture::from_file("assets/background.png").expect("Failed to load background texture");
    let mut background = Sprite::new();
    background.set_texture(&background_texture, false);

    // game_title definitions
    let game_title_texture =
        Texture::from_file("assets/gameTitle.png").expect("Failed to load background texture");
    let mut game_title = Sprite::new();
    game_title.set_texture(&game_title_texture, false);
    game_title.set_origin(Vector2f::new(266.0, 30.0));
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
    let mut score: i32 = 0;
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
    ship.set_position(Vector2f::new(200.0, 400.0));

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

    let enemy_texture = Texture::from_file("assets/enemyDefault.png").unwrap();
    let enemy_fast_texture = Texture::from_file("assets/enemyFast.png").unwrap();
    let enemy_armored_texture = Texture::from_file("assets/enemyArmored.png").unwrap();
    let mut enemies: Vec<Enemy> = Vec::new();

    let mut wave_number = 0;
    let mut textures: HashMap<EnemyType, &Texture> = HashMap::new();
    textures.insert(EnemyType::Default, &enemy_texture);
    textures.insert(EnemyType::Fast, &enemy_fast_texture);
    textures.insert(EnemyType::Armored, &enemy_armored_texture);

    let battery_texture_0 = Texture::from_file("assets/battery0.png").unwrap();
    let battery_texture_1 = Texture::from_file("assets/battery1.png").unwrap();
    let battery_texture_2 = Texture::from_file("assets/battery2.png").unwrap();
    let battery_texture_3 = Texture::from_file("assets/battery3.png").unwrap();
    let battery_texture_4 = Texture::from_file("assets/battery4.png").unwrap();
    let battery_texture_5 = Texture::from_file("assets/battery5.png").unwrap();

    let mut textures_battery: HashMap<i32, &Texture> = HashMap::new();
    textures_battery.insert(0, &battery_texture_0);
    textures_battery.insert(1, &battery_texture_1);
    textures_battery.insert(2, &battery_texture_2);
    textures_battery.insert(3, &battery_texture_3);
    textures_battery.insert(4, &battery_texture_4);
    textures_battery.insert(5, &battery_texture_5);
    let mut battery = Sprite::new();
    battery.set_texture(&battery_texture_0, false);
    battery.set_scale(3.0);
    battery.set_position(Vector2f::new(
        WIDTH as f32 - 64.0 * 3.0 / 2.0,
        HEIGHT as f32 - 24.0 * 3.0,
    ));

    let mut current_wave = spawn_wave(&textures, wave_number);
    // ----------------------- GAME OVER SCREEEN --------------------

    let game_over_text_texture =
        Texture::from_file("assets/gameOverText.png").expect("Failed to load projectile texture");
    let mut game_over_text = Sprite::new();
    game_over_text.set_texture(&game_over_text_texture, false);
    game_over_text.set_origin(Vector2f::new(53.5, 26.0));
    game_over_text.set_position(Vector2f::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0 - 150.0,
    ));
    game_over_text.set_scale(3.0);

    let play_again_button_texture = Texture::from_file("assets/buttonPlayAgain.png")
        .expect("Failed to load projectile texture");
    let mut play_again_button = Sprite::new();
    play_again_button.set_texture(&play_again_button_texture, false);
    play_again_button.set_origin(Vector2f::new(40.0, 12.0));
    play_again_button.set_position(Vector2f::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0 + 50.0,
    ));
    play_again_button.set_scale(4.0);

    let menu_button_texture =
        Texture::from_file("assets/menuButton.png").expect("Failed to load projectile texture");
    let mut menu_button = Sprite::new();
    menu_button.set_texture(&menu_button_texture, false);
    menu_button.set_origin(Vector2f::new(32.0, 12.0));
    menu_button.set_position(Vector2f::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0 + 180.0,
    ));
    menu_button.set_scale(4.0);

    //
    // =================== MAIN LOOP ====================
    //

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
                        if !has_game_started && !settings_opened {
                            let mouse_pos =
                                window.map_pixel_to_coords(Vector2i::new(x, y), &window.view());

                            if button_play.global_bounds().contains(mouse_pos) {
                                has_game_started = true;
                                button_play.set_texture(&button_play_off_texture, false);
                                wave_number = 0;
                                is_game_over = false;
                                bullets_availiable = 5;
                                ship.set_position(Vector2f::new(200.0, 400.0));
                                current_wave = spawn_wave(&textures, wave_number);
                            } else if button_settings.global_bounds().contains(mouse_pos) {
                                settings_opened = true;
                            }
                        } else if has_game_started && is_game_over {
                            let mouse_pos =
                                window.map_pixel_to_coords(Vector2i::new(x, y), &window.view());
                            if play_again_button.global_bounds().contains(mouse_pos) {
                                wave_number = 0;
                                is_game_over = false;
                                bullets_availiable = 5;
                                ship.set_position(Vector2f::new(200.0, 400.0));
                                current_wave = spawn_wave(&textures, wave_number);
                            } else if menu_button.global_bounds().contains(mouse_pos) {
                                has_game_started = false;
                                is_game_over = false;
                            }
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
                if enemy.health <= 0 {
                    enemy.alive = false;
                    score += enemy.enemy_type.points_gained();
                }
            }

            current_wave
                .enemies
                .retain(|e| e.sprite.position().y < HEIGHT as f32 && e.alive == true);

            if current_wave.enemies.is_empty() {
                wave_number += 1;
                current_wave = spawn_wave(&textures, wave_number);
            }

            for enemy in &current_wave.enemies {
                if ship
                    .global_bounds()
                    .intersection(&enemy.sprite.global_bounds())
                    != None
                {
                    is_game_over = true;
                }
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
                        enemy.health -= 1;
                        projectile.damage += 1;
                    }
                }
            }

            projectiles.retain(|p| {
                p.sprite.position().x > 0.0 && p.sprite.position().x < WIDTH as f32 && p.damage < 2
            });

            for projectile in &projectiles {
                window.draw(&projectile.sprite);
            }
            battery.set_texture(textures_battery[&(bullets_availiable as i32)], false);
            window.draw(&battery);
        } else {
            // game over
            let desktop_pos = mouse::desktop_position();
            let window_pos = window.position();
            let relative = Vector2f::new(
                desktop_pos.x as f32 - window_pos.x as f32,
                desktop_pos.y as f32 - window_pos.y as f32,
            );

            if play_again_button.global_bounds().contains(relative) {
                play_again_button.set_scale(4.5);
            } else {
                play_again_button.set_scale(4.0);
            }

            if menu_button.global_bounds().contains(relative) {
                menu_button.set_scale(4.5);
            } else {
                menu_button.set_scale(4.0);
            }

            if game_over_text.global_bounds().contains(relative) {
                game_over_text.set_scale(3.4);
            } else {
                game_over_text.set_scale(3.0);
            }

            window.draw(&background);
            window.draw(&game_over_text);
            window.draw(&play_again_button);
            window.draw(&menu_button);
        }
        window.display();
    }
}
