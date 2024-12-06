pub fn run(input: &str) -> anyhow::Result<String> {
    let lines = input.lines();

    // (first_num, second_num)
    let mut rules: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

    let mut result = 0;

    for line in lines {
        if line.contains("|") {
            let mut split = line.split("|");

            rules.push((
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            ));
        } else if line.contains(",") {
            let split = line.split(",");

            let mut update = vec![];

            for num in split {
                update.push(num.parse().unwrap());
            }

            updates.push(update);
        }
    }

    // dbg!(rules);
    // dbg!(updates);

    for update in updates {
        let mut valid = true;
        
        for (first_num, second_num) in &rules {
            let first_num_index = update.iter().position(|num| num == first_num);
            let second_num_index = update.iter().position(|num| num == second_num);

            if let (Some(first_num_index), Some(second_num_index)) =
                (first_num_index, second_num_index)
            {
                if first_num_index > second_num_index {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            result += update[update.len() / 2]
        }
    }

    // dbg!(result);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", run(input)?);
        Ok(())
    }
}
