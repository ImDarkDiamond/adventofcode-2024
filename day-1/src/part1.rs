pub fn run(input: &str) -> anyhow::Result<String> {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];

    let lines = input.lines();

    for line in lines {
        let mut split = line.split_whitespace();

        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    // dbg!(&left);
    // dbg!(&right);

    let distance: i64 = left
        .iter()
        .enumerate()
        .map(|(index, left)| (right.get(index).unwrap() - left).abs())
        .sum();

    // dbg!(distance);
    return Ok(distance.to_string());
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
        assert_eq!("11", run(input)?);
        Ok(())
    }
}
