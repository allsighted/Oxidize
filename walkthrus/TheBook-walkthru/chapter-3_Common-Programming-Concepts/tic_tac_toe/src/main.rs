use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, 
    style::{Color, SetForegroundColor, ResetColor, SetBackgroundColor},
    terminal::{self, ClearType},
};
use std::{io::{self, Write}, thread, time::Duration};

type Result<T> = std::result::Result<T, std::io::Error>;

fn main() -> Result<()> {
    // Setup terminal for raw mode
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    // Initialize grid with underscores
    let mut grid: Vec<String> = vec![String::from("_"); 9];
    
    // Game state
    let mut cursor_pos = 0; // Current position in the grid
    let mut current_player = "X"; // Start with player X
    let mut game_over = false;
    let mut winner = "";
    let mut winning_line = Vec::new(); // Store the winning indices
    
    loop {
        // Clear screen and redraw
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        
        // Display grid with cursor highlighting
        for row in 0..3 {
            for col in 0..3 {
                let idx = row * 3 + col;
                
                // Special coloring for winning line
                if winning_line.contains(&idx) {
                    execute!(stdout, SetBackgroundColor(Color::Green))?;
                } 
                // Cursor highlighting (only if not game over)
                else if idx == cursor_pos && !game_over {
                    execute!(stdout, SetBackgroundColor(Color::Grey))?;
                }
                
                // Color X red and O blue
                if grid[idx] == "X" {
                    execute!(stdout, SetForegroundColor(Color::Red))?;
                } else if grid[idx] == "O" {
                    execute!(stdout, SetForegroundColor(Color::Blue))?;
                }
                
                write!(stdout, " {} ", grid[idx])?;
                
                // Reset all styling
                execute!(stdout, ResetColor)?;
            }
            // Move to next line
            execute!(stdout, cursor::MoveTo(0, row as u16 + 1))?;
        }
        
        // Display game status
        execute!(stdout, cursor::MoveTo(0, 4))?;
        if game_over {
            if winner.is_empty() {
                write!(stdout, "Game over! It's a draw.")?;
            } else {
                if winner == "X" {
                    execute!(stdout, SetForegroundColor(Color::Red))?;
                } else {
                    execute!(stdout, SetForegroundColor(Color::Blue))?;
                }
                write!(stdout, "Player {} Wins!", winner)?;
                execute!(stdout, ResetColor)?;
            }
        } else {
            write!(stdout, "Current turn: ")?;
            if current_player == "X" {
                execute!(stdout, SetForegroundColor(Color::Red))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::Blue))?;
            }
            write!(stdout, "{}", current_player)?;
            execute!(stdout, ResetColor)?;
        }
        
        // Display controls
        execute!(stdout, cursor::MoveTo(0, 5))?;
        if game_over {
            write!(stdout, "Press Esc to quit or Enter to play again")?;
        } else {
            write!(stdout, "Controls: Arrow keys to move, Enter to place, Esc to quit")?;
        }
        
        stdout.flush()?;
        
        // Handle input
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if !game_over && cursor_pos >= 3 {
                        cursor_pos -= 3;
                    }
                },
                KeyCode::Down => {
                    if !game_over && cursor_pos < 6 {
                        cursor_pos += 3;
                    }
                },
                KeyCode::Left => {
                    if !game_over && cursor_pos % 3 > 0 {
                        cursor_pos -= 1;
                    }
                },
                KeyCode::Right => {
                    if !game_over && cursor_pos % 3 < 2 {
                        cursor_pos += 1;
                    }
                },
                KeyCode::Enter => {
                    if game_over {
                        // Reset the game
                        grid = vec![String::from("_"); 9];
                        current_player = "X";
                        game_over = false;
                        winner = "";
                        winning_line.clear();
                    } else if grid[cursor_pos] == "_" {
                        // Place the current player's mark
                        grid[cursor_pos] = current_player.to_string();
                        
                        // Check for win
                        if let Some(win_line) = check_win_with_line(&grid, current_player) {
                            // Animate winning line first
                            animate_winning_line(&mut stdout, &grid, &win_line, current_player)?;
                            
                            // Then set game state
                            game_over = true;
                            winner = current_player;
                            winning_line = win_line;
                            
                            // Finally animate the win message
                            animate_win(&mut stdout, winner)?;
                        } 
                        // Check for draw (all cells filled)
                        else if !grid.contains(&String::from("_")) {
                            game_over = true;
                        } 
                        // Switch players if game continues
                        else {
                            current_player = if current_player == "X" { "O" } else { "X" };
                        }
                    }
                },
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    
    // Cleanup
    terminal::disable_raw_mode()?;
    Ok(())
}

// Check for win and return the winning line indices
fn check_win_with_line(grid: &[String], player: &str) -> Option<Vec<usize>> {
    let player = player.to_string();
    
    // Check rows
    for row in 0..3 {
        if grid[row*3] == player && grid[row*3 + 1] == player && grid[row*3 + 2] == player {
            return Some(vec![row*3, row*3 + 1, row*3 + 2]);
        }
    }
    
    // Check columns
    for col in 0..3 {
        if grid[col] == player && grid[col + 3] == player && grid[col + 6] == player {
            return Some(vec![col, col + 3, col + 6]);
        }
    }
    
    // Check diagonals
    if grid[0] == player && grid[4] == player && grid[8] == player {
        return Some(vec![0, 4, 8]);
    }
    if grid[2] == player && grid[4] == player && grid[6] == player {
        return Some(vec![2, 4, 6]);
    }
    
    None
}

// Animate the winning line before showing the win message
fn animate_winning_line<W: Write>(stdout: &mut W, grid: &[String], win_line: &[usize], player: &str) -> Result<()> {
    let flash_count = 5;
    let flash_delay = Duration::from_millis(150);
    
    for _ in 0..flash_count {
        // Flash on
        draw_grid(stdout, grid, win_line, player, true)?;
        thread::sleep(flash_delay);
        
        // Flash off
        draw_grid(stdout, grid, &[], player, false)?;
        thread::sleep(flash_delay);
    }
    
    // Final state with winning line highlighted
    draw_grid(stdout, grid, win_line, player, true)?;
    
    Ok(())
}

// Draw the grid with or without highlighting the winning line
fn draw_grid<W: Write>(stdout: &mut W, grid: &[String], win_line: &[usize], current_player: &str, highlight_win: bool) -> Result<()> {
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    
    // Display grid
    for row in 0..3 {
        for col in 0..3 {
            let idx = row * 3 + col;
            
            // Highlight winning line if needed
            if highlight_win && win_line.contains(&idx) {
                execute!(stdout, SetBackgroundColor(Color::Green))?;
            }
            
            // Color X red and O blue
            if grid[idx] == "X" {
                execute!(stdout, SetForegroundColor(Color::Red))?;
            } else if grid[idx] == "O" {
                execute!(stdout, SetForegroundColor(Color::Blue))?;
            }
            
            write!(stdout, " {} ", grid[idx])?;
            
            // Reset all styling
            execute!(stdout, ResetColor)?;
        }
        // Move to next line
        execute!(stdout, cursor::MoveTo(0, row as u16 + 1))?;
    }
    
    // Display status
    execute!(stdout, cursor::MoveTo(0, 4))?;
    if highlight_win {
        if current_player == "X" {
            execute!(stdout, SetForegroundColor(Color::Red))?;
        } else {
            execute!(stdout, SetForegroundColor(Color::Blue))?;
        }
        write!(stdout, "Player {} is winning...", current_player)?;
        execute!(stdout, ResetColor)?;
    } else {
        write!(stdout, "                        ")?; // Clear the line
    }
    
    stdout.flush()?;
    
    Ok(())
}

// Matrix-style letter animation for win message
fn animate_win<W: Write>(stdout: &mut W, winner: &str) -> Result<()> {
    let message = format!("Player {} Wins!", winner);
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=<>?";
    let animation_steps = 15;
    let animation_delay = Duration::from_millis(50);
    
    // Position for the animation
    let y_pos = 4;
    
    for step in 0..animation_steps {
        execute!(
            stdout,
            cursor::MoveTo(0, y_pos),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        
        // Gradually reveal the actual message
        for (i, actual_char) in message.chars().enumerate() {
            let progress = (step as f32) / (animation_steps as f32);
            let char_reveal_point = (i as f32) / (message.len() as f32);
            
            if progress > char_reveal_point {
                // Character is revealed
                if winner == "X" {
                    execute!(stdout, SetForegroundColor(Color::Red))?;
                } else {
                    execute!(stdout, SetForegroundColor(Color::Blue))?;
                }
                write!(stdout, "{}", actual_char)?;
                execute!(stdout, ResetColor)?;
            } else {
                // Random character from the matrix
                let random_index = (step + i) % chars.len();
                let random_char = chars.chars().nth(random_index).unwrap_or('?');
                
                execute!(stdout, SetForegroundColor(Color::White))?;
                write!(stdout, "{}", random_char)?;
                execute!(stdout, ResetColor)?;
            }
        }
        
        stdout.flush()?;
        thread::sleep(animation_delay);
    }
    
    // Final highlight with background
    execute!(
        stdout,
        cursor::MoveTo(0, y_pos)
    )?;
    
    if winner == "X" {
        execute!(stdout, SetBackgroundColor(Color::Red), SetForegroundColor(Color::White))?;
    } else {
        execute!(stdout, SetBackgroundColor(Color::Blue), SetForegroundColor(Color::White))?;
    }
    
    write!(stdout, "{}", message)?;
    execute!(stdout, ResetColor)?;
    stdout.flush()?;
    
    // Pause for effect
    thread::sleep(Duration::from_millis(500));
    
    Ok(())
}
