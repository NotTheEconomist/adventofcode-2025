use day1::*;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(dial: &mut DialPosition, instructions: Vec<Instruction>) -> usize {
    instructions
        .into_iter()
        .filter_map(|ins| {
            dial.apply(ins);
            if dial.position() == 0 { Some(1) } else { None }
        })
        .sum()
}

fn solve_part2(dial: &mut DialPosition, instructions: Vec<Instruction>) -> usize {
    instructions.into_iter().map(|ins| dial.apply(ins)).sum()
}

fn main() {
    let mut dial = DialPosition::new(50);
    let instructions: Vec<Instruction> =
        INPUT.lines().flat_map(str::parse::<Instruction>).collect();
    let part1 = solve_part1(&mut dial, instructions.clone());
    println!("Part 1: {}", part1);
    dial.reset();  // The absence of this line cost me over an hour of my life
    let part2 = solve_part2(&mut dial, instructions);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use super::*;
    const INPUT: &str = include_str!("test_input.txt");
    static PARSED_INPUT: LazyLock<Vec<Instruction>> =
        LazyLock::new(|| INPUT.lines().flat_map(str::parse::<Instruction>).collect());

    #[test]
    fn test_solve_part1() {
        let mut dial = DialPosition::new(50);
        let instructions = PARSED_INPUT.clone();
        let got = solve_part1(&mut dial, instructions);
        assert_eq!(got, 3)
    }

    #[test]
    fn test_solve_part2() {
        let mut dial = DialPosition::new(50);
        let instructions = PARSED_INPUT.clone();
        let got = solve_part2(&mut dial, instructions);
        assert_eq!(got, 6);
    }
}
