#![allow(non_snake_case, unused_variables, dead_code, non_camel_case_types, unused_must_use, unused_imports)]

use macroquad::prelude::*;

const SCALE: i32 = 5;
const WINDOW_SIZE: (i32,i32) = (210 * SCALE,160 * SCALE);

const PLAYER_SPEED: f32 = 2.0 * SCALE as f32;
const PLAYER_MARGIN: f32 = 0.5;

const BALL_SPEED: f32 = 3.0;
const STARTING_BALL_POSITION: (f32,f32) = ((WINDOW_SIZE.0 / 2) as f32, (WINDOW_SIZE.1 / 2) as f32);

struct Player{
    position: (f32,f32),
    size: (f32,f32),
    bot: bool
}

impl Player{
    fn move_y(&mut self, amount: f32){
        self.position.1 += amount;
        if self.position.1 < PLAYER_MARGIN || self.position.1 + self.size.1 + PLAYER_MARGIN > WINDOW_SIZE.1 as f32{
            self.position.1 -= amount;
        }
    }

    fn run_bot(&mut self, ball: &Ball){
        let direction = (ball.position.1 - self.size.1 / 2.0) - self.position.1;
        if direction > -3. && direction < 3.{ return;}
        let direction = direction.signum();

        self.move_y(PLAYER_SPEED * direction);
    }

    fn update(&mut self, ball: &Ball){
        if self.bot{
            self.run_bot(ball);
            return;
        }
        if is_key_down(KeyCode::W) {
            self.move_y(-PLAYER_SPEED);
        }
        else if is_key_down(KeyCode::S){
            self.move_y(PLAYER_SPEED);
        }
    }
    fn draw(&self){
        draw_rectangle(
            self.position.0,
            self.position.1,
            self.size.0, 
            self.size.1,
         WHITE);
    }
}

struct Ball{
    position: (f32,f32),
    velocity: (f32,f32),
    radius: f32
}

impl Ball{
    fn move_x(&mut self){
        self.position.0 += self.velocity.0;
        if self.position.0 < 0. || self.position.0 > WINDOW_SIZE.0 as f32{
            self.position = STARTING_BALL_POSITION;
            self.reset_velocity();
        }
    }
    fn move_y(&mut self){
        if self.position.1 - self.radius < 0. || self.position.1 + - self.radius > WINDOW_SIZE.1 as f32{
            self.position.1 -= self.velocity.1;
            self.velocity.1 *= -1.;
        }
        else {
            self.position.1 += self.velocity.1;
        }
    }
    fn reset_velocity(&mut self){
        use ::rand::prelude::*;
        let mut rng = thread_rng();
        self.velocity.0 = (2 * rng.gen_bool(0.5) as i32 - 1) as f32 * BALL_SPEED;
        self.velocity.1 = (2 * rng.gen_bool(0.5) as i32 - 1) as f32 * BALL_SPEED;
    }

    fn check_collision_with(&mut self, other: &Player){
        if other.position.0 < self.position.0 && other.position.0 + other.size.0 > self.position.0{
            if other.position.1 < self.position.1 && other.position.1 + other.size.1 > self.position.1{
                self.velocity.0 *= -1.;
            }
        }
    }

    fn update(&mut self){
        self.move_x();
        self.move_y();
    }
    fn draw(&self){
        draw_circle(
            self.position.0,
            self.position.1,
            self.radius,
         WHITE);
    }
}

struct GameState{
    player_one: Player,
    player_two: Player,
    ball: Ball
}

impl GameState{
    fn new() -> GameState{
        let mut ball = Ball{
            position: STARTING_BALL_POSITION,
            velocity: (0.,0.),
            radius: 1.5 * SCALE as f32
        };
        ball.reset_velocity();

        GameState{
            player_one: Player{
                position: (20.,300.),
                size: (10.,24. * SCALE as f32),
                bot: false
            },
            player_two: Player{
                position: (WINDOW_SIZE.0 as f32 - 4. * SCALE as f32 - 10.,300.),
                size: (10.,24. * SCALE as f32),
                bot: true
            },
            ball
        }
    }

    fn update(&mut self){
        self.ball.update();
        self.player_one.update(&self.ball);
        self.player_two.update(&self.ball);
        self.ball.check_collision_with(&self.player_one);
        self.ball.check_collision_with(&self.player_two);
        
    }

    fn draw(&self){
        self.ball.draw();
        self.player_one.draw();
        self.player_two.draw();
    }

}

fn window_conf() -> Conf {
    Conf {
        window_title: "Ping Pong".to_owned(),
        window_width: WINDOW_SIZE.0,
        window_height: WINDOW_SIZE.1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameState::new();
    loop {
        clear_background(BLACK);
        game.update();
        game.draw();
        next_frame().await
    }
}