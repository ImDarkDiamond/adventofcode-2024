pub fn run(input: &str) -> anyhow::Result<String> {
    let lines = input.lines();

    let safe = lines
        .filter(|line| {
            let safe = is_safe(line);

            // dbg!(line, safe);

            safe
        })
        .count();

    // dbg!(safe);

    return Ok(safe.to_string());
}

// Determines if a line is safe:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
fn is_safe(line: &str) -> bool {
    let values: Vec<i64> = line
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let increasing = values.get(1) > values.get(0);

    for (i, line) in values.iter().enumerate() {
        let next = values.get(i + 1);

        if let Some(next) = next {
            let diff = if increasing { next - line } else { line - next };

            // dbg!(increasing, diff, line, next);

            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", run(input)?);
        Ok(())
    }
}
