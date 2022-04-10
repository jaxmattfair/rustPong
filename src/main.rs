use macroquad::prelude::*;

struct MainState {
    ball: Rect,
    x_vel: f32,
    y_vel: f32,
    top_paddle: Rect,
    bottom_paddle: Rect,

    top_score: usize,
    bottom_score: usize,
}

impl MainState {
    fn draw(&self) {
        //draws score
        draw_text(&self.top_score.to_string(), 15.0, 30.0, 36.0, WHITE);
        draw_text(&self.bottom_score.to_string(), 15.0, screen_height() - 15.0, 36.0, WHITE);

        //draws ball
        draw_rectangle(self.ball.x, self.ball.y, self.ball.w, self.ball.h, WHITE);

        //draws paddles
        draw_rectangle(
            self.top_paddle.x,
            self.top_paddle.y,
            self.top_paddle.w,
            self.top_paddle.h,
            WHITE,
        );
        draw_rectangle(
            self.bottom_paddle.x,
            self.bottom_paddle.y,
            self.bottom_paddle.w,
            self.bottom_paddle.h,
            WHITE,
        );
    }

    fn update(&mut self) {
        self.ball.x += self.x_vel;
        self.ball.y += self.y_vel;

        //Controls ball collision with screen sides
        if self.ball.right() >= screen_width() || self.ball.left() <= 0.0 {
            self.x_vel *= -1.0
        }
        //Controls ball collision with paddles
//NEED TO DEBUG COLLISION WHEN NOT HITTING PADDLE STRAIGHT ON
        if self.ball.overlaps(&self.top_paddle)
        || self.ball.overlaps(&self.bottom_paddle) {
            self.y_vel *= -1.0
        }

        //Iterates score if ball hits top or bottom
        if self.ball.top() <= 0.0 {
            self.bottom_score += 1;
        }
        if self.ball.bottom() >= screen_height() {
            self.top_score += 1;
        }

        //Ball respawns in center screen
//NEED TO ADD VELOCITY RANDOMIZER
        if self.ball.top() <= 0.0
        || self.ball.bottom() >= screen_height() {
            self.ball.x = screen_width() / 2.0;
            self.ball.y = screen_height() / 2.0;
        }

        //Paddle movements
        if is_key_down(KeyCode::Right) {
            self.top_paddle.x += 10.0
        }
        if is_key_down(KeyCode::Left) {
            self.top_paddle.x -= 10.0
        }
        if is_key_down(KeyCode::D) {
            self.bottom_paddle.x += 10.0
        }
        if is_key_down(KeyCode::A) {
            self.bottom_paddle.x -= 10.0
        }
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut main_state = MainState {
        //initializes ball position, size, and velocity
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        x_vel: 5.0,
        y_vel: -5.0,

        //initializes top and bottom paddle position and size
        top_paddle: Rect::new(screen_width() / 2.0, 15.0, 100.0, 15.0),
        bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 15.0, 100.0, 15.0),

        //initializes score
        top_score: 0,
        bottom_score: 0,
    };
    // equivalent to while(true)
    loop {
        clear_background(BLACK);

        main_state.draw();
        main_state.update();

        // let macroquad handle frame times,
        // input updates, etc
        next_frame().await
    }
}
