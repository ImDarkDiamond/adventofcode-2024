#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let mut grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split("").filter(|c| !c.is_empty()).collect())
        .collect();

    let mut moves = 1;

    let height = grid.len();
    let width = grid[0].len();

    // seems the guard always starts facing up so i've just hardcoded that
    let mut facing = Direction::Up;

    // (row, col)
    let mut current_position = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            // Found the guard!
            if *col == "^" {
                current_position = (row_index, col_index);
            }
        }
    }

    // Set the guards unique position (line 15 already counts it)
    grid[current_position.0][current_position.1] = "X";

    loop {
        // (row, col)
        let next_coords = match facing {
            Direction::Up => (current_position.0 - 1, current_position.1),
            Direction::Down => (current_position.0 + 1, current_position.1),
            Direction::Left => (current_position.0, current_position.1 - 1),
            Direction::Right => (current_position.0, current_position.1 + 1),
        };

        if next_coords.0 >= width || next_coords.1 >= height {
            break;
        }

        // dbg!(next_coords);
        let next = grid[next_coords.0][next_coords.1];

        if next == "#" {
            facing = match facing {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            };

            // dbg!(&facing);

            // println!("hit something! {next_coords:?}");
        } else {
            if grid[next_coords.0][next_coords.1] != "X" {
                moves += 1;
            }

            grid[next_coords.0][next_coords.1] = "X";
            current_position = next_coords;
        }

        // println!("\nMoves: {moves}\n{}", grid.iter().map(|v| v.join("")).collect::<Vec<_>>().join("\n"));
        
        // dbg!(moves);
        // dbg!(&grid);
    }

    // dbg!(moves);

    Ok(moves.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", run(input)?);
        Ok(())
    }
}

// ....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...
