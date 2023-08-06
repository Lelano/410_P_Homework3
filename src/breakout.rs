#![allow(unused)]

use libm::*;
//use rtt_target::rprintln;

pub struct GameState {
    blocks: [u8; 5],
    ball_position: [f32; 2],
    ball_direction: [f32; 2],
    ball_velocity: f32,
    paddle_position: f32,
    paddle_width: f32,
    paddle_velocity: f32,
    ball_count: u8,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            blocks: [2; 5],
            ball_position: [2.0, 3.0],
            ball_direction: [0.8, 0.2],
            ball_velocity: 5.0,
            paddle_position: 2.5,
            paddle_width: 1.8,
            paddle_velocity: 5.0,
            ball_count: 3,
        }
    }
}

pub type Raster = [[u8; 5]; 5];

impl GameState {
    pub fn step(&mut self, raster: &mut Raster, tick: u16) -> bool {
        let tick = 0.001 * tick as f32;

        let coords = self.ball_position
            .iter_mut()
            .zip(self.ball_direction.iter_mut());
        for (x, dx) in coords {
            *x = (*x + *dx * self.ball_velocity * tick).clamp(0.0, 5.0);
        }
        let [r, c] = self.ball_position;
        let [ur, uc] = [r, c].map(|x| (floorf(x + 0.5) as usize).clamp(0, 4));

        let [ref mut dr, ref mut dc] = self.ball_direction;
        if !(0.001..=4.999).contains(&r) {
           *dr = -*dr;
        } else if ur == 3 && self.blocks[uc] > 0 {
            self.blocks[uc] -= 1;
            *dr = -*dr;
        } else {
            let pw = self.paddle_width;
            let pp = self.paddle_position;
            if ur == 0 && fabsf(pp - c) < 0.5 * pw {
                *dr = -*dr;
            }
        }
        if !(0.001..=4.999).contains(&c) {
            *dc = -*dc;
        }
        self.ball_direction = [*dr, *dc];
        self.ball_position = [r, c];

        //rprintln!("{} {}", r, c);
        for c in 0..5 {
            if self.blocks[c] > 0 {
                raster[3][c] = 9;
            }

            if fabsf(c as f32 - pp) < 0.5 * pw {
                raster[0][c] = 9;
            }
        }
        raster[ur][uc] = 9;
        self.blocks.iter().all(|&b| b == 0)
    }
}
