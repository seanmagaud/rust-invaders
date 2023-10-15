use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::Shot, invaders::Invaders};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2, // Initialize the player's position in the middle of the screen.
            y: NUM_ROWS - 1, // Initialize the player's position in the bottom of the screen.
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            // Check if the current x position is greater than 0 to avoid going off the left edge of the screen.
            self.x -= 1; // Move the player one unit to the left.
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            // Check if the current x position is less than the screen width minus 1 to avoid going off the right edge of the screen.
            self.x += 1; // Move the player one unit to the right.
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            // Check if the player has less than 2 shots on the screen.
            self.shots.push(Shot::new(self.x, self.y -1)); // Create a new shot and add it to the player's shots vector.
            return true;
        }
        return false;
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut(){
            // The difference between iter_mut() and iter().map() is 
            // that iter_mut() will modify the actual collection instead of creating a new one
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    shot.explode();
                    hit_something = true;
                }
            }
        }
        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"; // Draw the player as the letter "A".
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}