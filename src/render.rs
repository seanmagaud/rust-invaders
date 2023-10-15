use crossterm::{style::{SetBackgroundColor, Color}, terminal::{ClearType, Clear}, QueueableCommand, cursor::MoveTo};
use std::io::{Stdout, Write};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
     stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
     stdout.queue(Clear(ClearType::All)).unwrap();  
     stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

     // Iterate over each column in the current frame.
     for (x, col) in curr_frame.iter().enumerate() {
        // Iterate over each element in the current column.
        for (y, s) in col.iter().enumerate() {
            // Check if the current character differs from the previous frame or if "force" is true.
            if *s != last_frame[x][y] || force {
                // If the condition is met, execute the following commands:

                // Move the terminal's cursor to position (x, y).
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                // Print the character "s" at the current position.
                print!("{}", s);
            }
        }
    }

    // Flush the terminal's output buffer to ensure all commands are executed.
    stdout.flush().unwrap();
}