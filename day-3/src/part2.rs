use regex::Regex;

pub fn run(input: &str) -> anyhow::Result<String> {
    // I hate regex. So. Much. Can't be bothered to figure out how to do this with just one regex so 2 it is lol
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))")?;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut sums: Vec<i64> = vec![];
    let mut mul_enabled = true;

    for capture in re.captures_iter(input) {
        let (v, [_]) = capture.extract();

        match v {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            other => {
                if mul_enabled {
                    let (_, [num1, num2]) = mul_re.captures(other).unwrap().extract();

                    sums.push(num1.parse::<i64>()? * num2.parse::<i64>()?);
                }
            }
        }
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", run(input)?);
        Ok(())
    }
}
