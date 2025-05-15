fn main() {
    let mut grid = [[0; 3]; 3];
    run_repl(&mut grid);
}

fn display_cell(val: i32) -> char {
    match val {
        0 => '_',
        1 => 'X',
        2 => 'O',
        _ => '?',
    }
}

fn run_repl(grid: &mut [[i32; 3];3]) {
    use std::io::{self, Write};

    loop {
        for (i, &row) in grid.iter().enumerate() {
            for (i2, &point) in row.iter().enumerate() {
                let cell_value = display_cell(point);
                
                if i2 == 2 {
                    print!("{cell_value}");
                } else {
                    print!("{cell_value} | ");
                }
            }
            print!("\n");
            if i < 2 {
                // println!("---------");
            }
        }

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split('/').collect();
        
        if parts.len() != 2 {
            println!("Invalid input! Use format: position/symbol (e.g., 2/X)");
            continue;
        }

        // Parse position (0-8)
        let position = match parts[0].trim().parse::<usize>() {
            Ok(pos) if pos < 9 => pos,
            _ => {
                println!("Invalid position! Must be a number between 0-8");
                continue;
            }
        };

        // Parse symbol (X or O)
        let value = match parts[1].trim() {
            "X" | "x" => 1,
            "O" | "o" => 2,
            "_" => 0, // Allow clearing a cell
            _ => {
                println!("Invalid symbol! Use X, O, or _");
                continue;
            }
        };

        // Convert position to row and column
        let row = position / 3;
        let col = position % 3;
        
        // Update the grid
        grid[row][col] = value;
    }
}
