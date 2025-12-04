use day3::BatteryBank;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(banks: Vec<BatteryBank<2, u32>>) -> u64 {
    banks.iter().map(BatteryBank::joltage).sum()
}

fn solve_part2(banks: Vec<BatteryBank<12, u64>>) -> u64 {
    banks.iter().map(BatteryBank::joltage).sum()
}

fn main() -> anyhow::Result<()> {
    let banks = INPUT.lines().map(str::parse::<BatteryBank<2, u32>>).collect::<anyhow::Result<Vec<_>>>()?;
    let part1 = solve_part1(banks);
    println!("Day 3 part1: {}", part1);
    let banks = INPUT.lines().map(str::parse).collect::<anyhow::Result<Vec<_>>>()?;
    let part2 = solve_part2(banks);
    println!("Day 3 part2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part1() {
        let banks = INPUT.lines().flat_map(str::parse).collect();
        assert_eq!(solve_part1(banks), 357)
    }
    #[test]
    fn test_solve_part2() {
        let banks = INPUT.lines().flat_map(str::parse).collect();
        assert_eq!(solve_part2(banks), 3121910778619)
    }
}
