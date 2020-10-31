
use ggez::{graphics, Context, ContextBuilder, GameResult, input};
use ggez::event::{self, EventHandler, KeyCode};
use rand::prelude::*;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
		.build()
		.expect("aieee, could not create ggez context!");
    let mut my_game = MyGame::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

fn draw_block(mb: &mut graphics::MeshBuilder, pos: (i32, i32), size: (i32, i32), color: graphics::Color) {
    mb.rectangle(
        graphics::DrawMode::fill(), 
        [pos.0 as f32, pos.1 as f32, size.0 as f32, size.1 as f32].into(), 
        color,
        );
}

fn normalize(input_vector: (f32, f32)) -> (f32, f32) {
    if input_vector.0 != 0. && input_vector.1 != 0. {
        let input_vector_size = (input_vector.0.powi(2) + input_vector.1.powi(2)).sqrt();
        return (
            input_vector.0 / input_vector_size,
            input_vector.1 / input_vector_size
        );
    } else {
        return input_vector;
    }
}

fn collision(position: (i32, i32), position2: (i32, i32), size: (i32, i32), size2: (i32, i32)) -> bool {
    return position.0 + size.0 > position2.0 
    && position.1 + size.1 > position2.1
    && position.0 < position2.0 + size2.0
    && position.1 < position2.1 + size2.1;
}

struct Player {
    position: (i32, i32),
    size: (i32, i32),
    speed: f32,
    bullets: Vec<Bullet>,
    shoot_timer: f32,
}

impl Player {
    fn update(&mut self, ctx: &mut Context) {
        let delta = ggez::timer::delta(&ctx).as_secs_f32();
        let mut input_vector: (f32, f32) = (0., 0.);
        
        
        if input::keyboard::is_key_pressed(ctx, KeyCode::D) {
            input_vector.0 = 1.;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            input_vector.0 = -1.;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            input_vector.1 = -1.;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            input_vector.1 = 1.;
        }

        input_vector = normalize(input_vector);

        // println!("{}, {}, {}", input_vector.0, input_vector.1, delta);

        self.position.0 += (input_vector.0 * self.speed * delta) as i32;
        self.position.1 += (input_vector.1 * self.speed * delta) as i32;
        
        self.shoot_timer += delta;
        if input::mouse::button_pressed(ctx, event::MouseButton::Left) && self.shoot_timer > 0.5 {
            let mouse = input::mouse::position(ctx);
            let direction = (mouse.x  - self.position.0 as f32, mouse.y - self.position.1 as f32);
            let direction = normalize(direction);

            let new_bullet = Bullet {
                position: self.position,
                size: (10, 10),
                direction: direction,
                speed: 1000.,
            };
            self.bullets.insert(0, new_bullet);

            self.shoot_timer = 0.;
        }

        for i in 0..self.bullets.len() {
            self.bullets[i].update(ctx);
        }

        //despawn bullets
        let mut i = 0;
        while i != self.bullets.len() {
            if self.bullets[i].speed == 0. {
                self.bullets.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

struct Bullet {
    position: (i32, i32),
    size: (i32, i32),
    direction: (f32, f32),
    speed: f32,
}

impl Bullet {
    fn update(&mut self, ctx: &mut Context) {
        let delta = ggez::timer::delta(&ctx).as_secs_f32();

        self.position.0 += (self.direction.0 * self.speed * delta) as i32;
        self.position.1 += (self.direction.1 * self.speed * delta) as i32;

        let window = graphics::size(ctx);
        let window = (window.0 as i32, window.1 as i32);
        if self.position.0 > window.0 || self.position.0 < 0 || self.position.1 > window.1 || self.position.1 < 0 {
            self.speed = 0.;
        }
    }
}

struct Enemy {
    position: (i32, i32),
    size: (i32, i32),
    speed: f32,
}

impl Enemy {
    fn update(&mut self, ctx: &mut Context, player: &Player) {
        let delta = ggez::timer::delta(&ctx).as_secs_f32();
        let direction = (player.position.0 - self.position.0, player.position.1 - self.position.1);
        let direction = normalize((direction.0 as f32, direction.1 as f32));

        self.position.0 += (direction.0 * self.speed * delta) as i32;
        self.position.1 += (direction.1 * self.speed * delta) as i32;

        //player collision
        if collision(self.position, player.position, self.size, player.size) {
            self.speed = 0.;
        }

        //bullet collision
        for bullet in player.bullets.iter() {
            if collision(self.position, bullet.position, self.size, bullet.size) {
                self.speed = 0.;
            }
        }
    }
}


struct MyGame {
    player: Player,
    enemies: Vec<Enemy>,
    spawn_enemy_timer: f32,
    score: i32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            player: Player {
                position: (0, 0),
                size: (40, 40),
                speed: 300.,
                bullets: vec![],
                shoot_timer: 0.,
            },
            enemies: vec![],
            spawn_enemy_timer: 0.,
            score: 0,
        }
    }
}


impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = ggez::timer::delta(&ctx).as_secs_f32();

        self.player.update(ctx);

        self.spawn_enemy_timer += delta;
        if self.spawn_enemy_timer > 1. {
            self.spawn_enemy_timer = 0.;
            self.enemies.insert(0, Enemy {
                position: [(-50, -50), (800 + 50, -50), (-50, 600 + 50), (8000 + 50, 600 + 50)][rand::thread_rng().gen_range(0, 4)],
                size: (40, 40),
                speed: 200.,
            });
        
        }
        for i in 0..self.enemies.len() {
            self.enemies[i].update(ctx, &self.player);
        }
        
        for enemy in self.enemies.iter() {
            if collision(self.player.position, enemy.position, self.player.size, enemy.size) {
                self.score = 0;
            }
        }
        //despawn enemies
        let mut i = 0;
        while i != self.enemies.len() {
            if self.enemies[i].speed == 0. {
                self.enemies.remove(i);
                self.score += 1;
            } else {
                i += 1;
            }
        }

        //player - enemy colllision

        graphics::set_window_title(ctx, &self.score.to_string());

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0., 0.5, 0.5, 1.));
        
        let mut mb = graphics::MeshBuilder::new();

        draw_block(&mut mb, self.player.position, self.player.size, [0.5, 0., 0.5, 1.].into());
        for bullet in self.player.bullets.iter() {
            draw_block(&mut mb, bullet.position, bullet.size, graphics::Color::new(0., 0., 0., 1.));
        }
        for enemy in self.enemies.iter() {
            draw_block(&mut mb, enemy.position, enemy.size, graphics::Color::new(0., 0., 0., 1.));
        }

        let mb = mb.build(ctx).unwrap();
        graphics::draw(ctx, &mb, graphics::DrawParam::new())?;

        graphics::present(ctx)
    }
}