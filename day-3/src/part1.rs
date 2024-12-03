use regex::Regex;

pub fn run(input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut sums: Vec<i64> = vec![];

    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        sums.push(num1.parse::<i64>()? * num2.parse::<i64>()?);
    }

    let result = sums.iter().sum::<i64>().to_string();

    // dbg!(&result);

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", run(input)?);
        Ok(())
    }
}
