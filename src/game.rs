//! Breakout game proper. Assumes a 5×5 LED array in row-major order,
//! with 0, 0 at top left and integer brightness *b* with `0 ≤ b ≤ 9`.

use crate::*;

use libm::*;
//use rtt_target::rprintln;

/// State variables of current game.
pub struct GameState {
    blocks: [u8; 5],
    ball_position: [f32; 2],
    ball_direction: [f32; 2],
    ball_velocity: f32,
    paddle_position: f32,
    paddle_width: f32,
    ball_count: u8,
}

impl GameState {
    // Make a new starting `GameState` with game velocities
    // determined by the given `tick` (in milliseconds). See
    // [set_tick()] and [reset_ball()] for game state
    // updates.
    pub fn new(tick: u16) -> Self {
        let mut result = Self {
            blocks: [2; 5],
            ball_position: [0.0, 0.0],
            ball_direction: [0.0, 0.0],
            ball_velocity: 0.0,
            paddle_position: 2.5,
            paddle_width: 2.1,
            ball_count: 3,
        };
        result.reset_ball();
        result.set_tick(tick);
        result
    }

    pub fn set_tick(&mut self, tick: u16) {
        let tick = 0.001 * tick as f32;
        self.ball_velocity = (5.0 * tick).min(0.75);
    }

    fn reset_ball(&mut self) {
        self.ball_position = [3.0, 3.0];
        self.ball_direction = [-sinf(1.2), cosf(1.2)];
    }

    pub fn step(
        &mut self,
        raster: &mut Raster,
        knob: Option<f32>,
    ) -> bool {
        let coords = self.ball_position
            .iter_mut()
            .zip(self.ball_direction.iter_mut());
        for (x, dx) in coords {
            *x = (*x + *dx * self.ball_velocity).clamp(0.0, 5.0);
        }
        let [r, c] = self.ball_position;
        let [ur, uc] = [r, c].map(|x| (floorf(x + 0.5) as usize).clamp(0, 4));

        let pw = self.paddle_width;
        let mut pp = self.paddle_position;
        if let Some(bs) = knob {
            pp = 5.0 * bs;
            self.paddle_position = pp;
        }

        let [ref mut dr, ref mut dc] = self.ball_direction;
        let ball_count = self.ball_count;
        if knob.is_none() && r > 4.25 && *dr > 0.0 {
            if self.ball_count > 0 {
                self.ball_count -= 1;
            } else {
                return true;
            }
        } else if !(0.001..=4.999).contains(&r) {
            *dr = -*dr;
        } else if ur == 1 && self.blocks[uc] > 0 {
            self.blocks[uc] -= 1;
            *dr = -*dr;
        } else if r < 1.5 && *dr > 0.0 && fabsf(pp - c) < 0.5 * pw {
            *dr = -*dr;
        }
        if !(0.001..=4.999).contains(&c) {
            *dc = -*dc;
        }
        if ball_count == self.ball_count {
            self.ball_direction = [*dr, *dc];
            self.ball_position = [r, c];
            raster[ur][uc] = 9;
        } else {
            self.reset_ball();
        }

        //rprintln!("{} {}", r, c);
        for c in 0..5 {
            if self.blocks[c] > 0 {
                raster[1][c] = 9;
            }

            if fabsf(c as f32 - pp) < 0.5 * pw {
                raster[4][c] = 9;
            }
        }
        self.blocks.iter().all(|&b| b == 0)
    }
}
