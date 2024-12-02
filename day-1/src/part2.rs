pub fn run(input: &str) -> anyhow::Result<String> {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    let lines = input.lines();

    for line in lines {
        let mut split = line.split_whitespace();

        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    }

    let mut total = 0;

    for v in left {
        let in_right = right.iter().filter(|r_v| **r_v == v).count();

        total += v * in_right;
    }

    // dbg!(total);

    return Ok(total.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", run(input)?);
        Ok(())
    }
}
