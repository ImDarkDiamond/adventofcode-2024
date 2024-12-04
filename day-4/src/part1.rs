// I don't know how to do this.
// I used the algorithm from here:
// https://www.geeksforgeeks.org/search-a-word-in-a-2d-grid-of-characters/#expected-approach-exploring-all-cells-omnk-time-and-o1-space
// Thank you people who are smarter than me for figuring this out so I could convert the code to rust.

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split("").filter(|c| !c.is_empty()).collect())
        .collect();

    // dbg!(&grid);

    let results = search_word(grid, "XMAS");

    // dbg!(results);

    return Ok(results.to_string());
}

fn search_word(grid: Vec<Vec<&str>>, word: &str) -> i32 {
    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let x = [-1, -1, -1, 0, 0, 1, 1, 1];
    let y = [-1, 0, 1, -1, 1, -1, 0, 1];

    let mut count = 0;

    for i in 0..height {
        for j in 0..width {
            for k in 0..8 {
                if find_word(0, word, &grid, i as i32, j as i32, x[k], y[k]) {
                    count = count + 1;
                }
            }
        }
    }

    return count;
}

fn find_word(
    index: usize,
    word: &str,
    grid: &Vec<Vec<&str>>,
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
) -> bool {
    let word_chars: Vec<char> = word.chars().collect();

    if index == word_chars.len() {
        return true;
    }

    if valid_coord(x, y, grid.len() as i32, grid[0].len() as i32)
        && word_chars[index] == grid[x as usize][y as usize].chars().next().unwrap()
    {
        return find_word(index + 1, word, grid, x + dir_x, y + dir_y, dir_x, dir_y);
    }

    false
}

fn valid_coord(x: i32, y: i32, rows: i32, cols: i32) -> bool {
    x >= 0 && x < rows && y >= 0 && y < cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", run(input)?);
        Ok(())
    }
}
