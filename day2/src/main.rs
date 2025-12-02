use day2::{Input, is_valid, is_valid2};

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: Input) -> u64 {
    input.into_iter().filter(|&id| !is_valid(id)).sum()
}

fn solve_part2(input: Input) -> u64 {
    input.into_iter().filter(|&id| !is_valid2(id)).sum()
}

fn main() -> anyhow::Result<()> {
    let input: Input = INPUT.parse()?;
    let part1 = solve_part1(input.clone());
    println!("day2 part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("day2 part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part1() {
        let input: Input = INPUT.parse().unwrap();
        assert_eq!(solve_part1(input), 1227775554)
    }
    #[test]
    fn test_solve_part2() {
        let input: Input = INPUT.parse().unwrap();
        assert_eq!(solve_part2(input), 4174379265)
    }
}
