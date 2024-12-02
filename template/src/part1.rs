pub fn run(_input: &str) -> anyhow::Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "";
        assert_eq!("", run(input)?);
        Ok(())
    }
}
