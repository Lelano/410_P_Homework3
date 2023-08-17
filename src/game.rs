//! Breakout game proper. Assumes a 5×5 LED array in row-major order,
//! with 0, 0 at top left and integer brightness *b* with `0 ≤ b ≤ 9`.

use crate::*;

use libm::*;
//use rtt_target::rprintln;

/// State variables of current game.
pub struct GameState {
    /// Row of blocks to be broken out.
    blocks: [u8; 5],
    /// Ball position in range 0.0..5.0.
    ball_position: [f32; 2],
    /// Ball direction as unit vector.
    ball_direction: [f32; 2],
    /// Ball velocity in pixels per tick. Should
    /// be less than 0.7 to ensure that game code
    /// works.
    ball_velocity: f32,
    /// Paddle position in range 0.0..1.0.
    paddle_position: f32,
    /// Paddle width as fraction of display width in range
    /// 0.0..1.0.
    paddle_width: f32,
    /// Number of remaining balls.
    ball_count: u8,
}

impl GameState {
    /// Make a new starting `GameState` with game velocities
    /// determined by the given `tick` (in milliseconds).
    pub fn new(tick: u16) -> Self {
        // Set up the base structure.
        let mut result = Self {
            blocks: [2; 5],
            ball_position: [0.0, 0.0],
            ball_direction: [0.0, 0.0],
            ball_velocity: 0.0,
            paddle_position: 2.5,
            paddle_width: 2.1, //Was 2.1
            ball_count: 3,
        };

        // Mutate to get initial (resettable) state.
        result.reset_ball();
        result.set_tick(tick);

        result
    }

    /// Change the tick time. Alter associated velocities.
    pub fn set_tick(&mut self, tick: u16) {
        let tick = 0.001 * tick as f32;
        self.ball_velocity = (5.0 * tick).min(0.75);
    }

    /// Put things to the start for the next ball.
    fn reset_ball(&mut self) {
        self.ball_position = [3.0, 3.0];
        self.ball_direction = [-sinf(1.2), cosf(1.2)];
    }

    /// Run one step of the game. Requires a display raster
    /// to draw on.  If the given knob value is `None` the
    /// step is taken in "attract mode", in which the ball
    /// bounces at the bottom rather than being lost.
    /// Otherwise, the knob position is given as a fraction
    /// of display width 0..1. Returns `true` if the game is
    /// over, and `false` otherwise.
    pub fn step(&mut self, raster: &mut Raster, knob: Option<f32>) -> bool {
        // Move the ball.
        let coords = self
            .ball_position
            .iter_mut()
            .zip(self.ball_direction.iter_mut());
        for (x, dx) in coords {
            *x = (*x + *dx * self.ball_velocity).clamp(0.0, 5.0);
        }
        let [r, c] = self.ball_position;
        let [ur, uc] = [r, c].map(|x| (floorf(x + 0.5) as usize).clamp(0, 4));

        // Move the paddle and record its new position.
        let pw = self.paddle_width;
        let mut pp = self.paddle_position;
        
        if let Some(bs) = knob {
            pp = 5.0 * bs;
            self.paddle_position = pp;
        }

        // Handle bounces, lost balls, etc.
        let [ref mut dr, ref mut dc] = self.ball_direction;
        let ball_count = self.ball_count;
        if knob.is_some() && r > 4.25 && *dr > 0.0 {
            // Lost the ball.
            if self.ball_count > 0 {
                self.ball_count -= 1;
            } else {
                // Lost the game.
                return true;
            }
        } else if !(0.001..=4.999).contains(&r) {
            // Top or bottom edge bounce.
            *dr = -*dr;
        } else if ur == 1 && self.blocks[uc] > 0 {
            // Block hit.
            self.blocks[uc] -= 1;
            *dr = -*dr;
            //Beep
            beep();

        } else if r < 1.5 && *dr > 0.0 && fabsf(pp - c) < 0.5 * pw {
            // Paddle bounce.
            *dr = -*dr;
        }
        if !(0.001..=4.999).contains(&c) {
            // Left or right edge bounce.
            *dc = -*dc;
        }
        if ball_count == self.ball_count {
            // Record ball position and render ball.
            self.ball_direction = [*dr, *dc];
            self.ball_position = [r, c];
            raster[ur][uc] = 9;
        } else {
            // Reset ball position. Do not render ball.
            self.reset_ball();
        }

        // Render blocks and paddle.
        for c in 0..5 {
            if self.blocks[c] > 0 {
                if self.blocks[c] == 1 {
                    //Block is half life.
                    raster[1][c] = 6;
                } else {
                    raster[1][c] = 9;
                }
            }

            if fabsf(c as f32 - pp) < 0.5 * pw {
                raster[4][c] = 9;
            }
        }

        // Won iff all blocks are destroyed.
        self.blocks.iter().all(|&b| b == 0)
    }
}
