use std::{error::Error, time::{Duration, Instant}, sync::mpsc, thread};
use crossterm::{terminal::{self, LeaveAlternateScreen, EnterAlternateScreen}, cursor::{Hide, Show}, ExecutableCommand, event::{KeyCode, self, Event}};
use invaders::{frame::{self, Drawable}, render, player::Player, invaders::Invaders};
use rusty_audio::Audio;
use std::io;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; // Enable raw mode for the terminal.
    stdout.execute(EnterAlternateScreen)?; // Switch to an alternate screen buffer.
    let _ = stdout.execute(Hide); // Hide the cursor.
    
    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();

    // Spawn a new thread for rendering
    let render_handle = thread::spawn(move || {
        // Initialize the last frame and stdout
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        // Render an initial frame
        render::render(&mut stdout, &last_frame, &last_frame, true);

        // Continuously render frames received via the render_rx channel
        while let Ok(curr_frame) = render_rx.recv() {
            // Render the game frame with updated stdout and frames
            render::render(&mut stdout, &last_frame, &curr_frame, false);

            // Update the last frame for the next iteration
            last_frame = curr_frame;
        }
    });

    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        //Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();
        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left | KeyCode::Char('q') | KeyCode::Char('a') => player.move_left(),
                    KeyCode::Right | KeyCode::Char('e') | KeyCode::Char('d')=> player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter=> {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    KeyCode::Esc => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame.clone()).unwrap();
        thread::sleep(Duration::from_millis(1)); // Delay for a short time to control the game speed.

        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }


    // Cleanup and finalize
    drop(render_tx); // Drop the sender to signal the render thread to exit.
    render_handle.join().unwrap(); // Wait for the render thread to finish.
    audio.wait(); // Wait for audio playback to finish.
    stdout.execute(Show)?; // Show the cursor.
    stdout.execute(LeaveAlternateScreen)?; // Switch back from the alternate screen.
    terminal::disable_raw_mode()?; // Disable raw mode for the terminal.

    Ok(())
}
