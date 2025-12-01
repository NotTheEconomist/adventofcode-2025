use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Instruction {
    Left(usize),
    Right(usize),
}

impl Instruction {
    pub fn full_rotations(&self) -> usize {
        match self {
            Self::Left(n) => n / 100,
            Self::Right(n) => n / 100,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("L", magnitude) => {
                let magnitude = magnitude.parse::<usize>().map_err(|_| String::from("Bad input"))?;
                Ok(Self::Left(magnitude))
            },
            ("R", magnitude) => {
                let magnitude = magnitude.parse::<usize>().map_err(|_| String::from("Bad input"))?;
                Ok(Self::Right(magnitude))
            },
            _ => Err(String::from("Bad input")),
        }
    }
}

pub struct DialPosition(usize);

impl DialPosition {
    pub fn new(position: usize) -> Self {
        Self(position)
    }
    pub fn reset(&mut self) {
        self.0 = 50
    }
    pub fn position(&self) -> usize {
        self.0
    }
    pub fn times_crossing_zero(&self, ins: Instruction) -> usize {
        let tail_rotate = match ins {
            Instruction::Left(n) if self.0 < n => 1usize,
            Instruction::Right(n) if self.0 + n >= 100 => 1usize,
            _ => 0usize,
        };
        tail_rotate + ins.full_rotations()
    }

    /// Apply an instruction to the dial, returning the number of times this instruction caused
    /// the dial to pass zero.
    ///
    /// # Examples:
    /// ```
    /// use day1::{Instruction, DialPosition};
    ///
    /// // Rotate right half a turn, crossing to zero
    /// let mut dial = DialPosition::new(50);
    /// let zeroes = dial.apply(Instruction::Right(50));
    /// assert_eq!(zeroes, 1);
    /// assert_eq!(dial.position(), 0);
    /// // Rotate left two-and-a-half times
    /// assert_eq!(dial.apply(Instruction::Left(250)), 2);
    /// assert_eq!(dial.position(), 50);
    /// ```
    pub fn apply(&mut self, ins: Instruction) -> usize {
        let mut spin_count = 0usize;
        spin_count += ins.full_rotations();
        let new_pos = match ins {
            Instruction::Left(n) => {
                // self.0 > 0 is a weird condition here, but catches when
                // we START at zero and spin left, which is not CROSSING zero.
                if self.0 > 0 && self.0 <= n % 100 {
                    spin_count += 1;
                }
                (self.0 + 100 - n % 100) % 100
            }
            Instruction::Right(n) => {
                // We don't need an equivalent self.0 < 100 here, since
                // modular arithmetic already provides that assertion
                if self.0 + n % 100 >= 100 {
                    spin_count += 1;
                }
                (self.0 + n % 100) % 100
            }
        };
        self.0 = new_pos;
        spin_count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! L {
        ($n:literal) => {
            Instruction::Left($n)
        };
    }

    macro_rules! R {
        ($n: literal) => {
            Instruction::Right($n)
        };
    }

    #[test]
    fn test_full_rotations() {
        let instruction: Instruction = R![1000];
        assert_eq!(instruction.full_rotations(), 10)
    }

    #[test]
    fn test_dial_spinning() {
        let instruction = R![1000];
        let mut dial = DialPosition::new(50);
        let zeroes = dial.apply(instruction);
        assert_eq!(zeroes, 10);

        let instruction = L![1000];
        let zeroes = dial.apply(instruction);
        assert_eq!(zeroes, 10);

        let zeroes = dial.apply(R![49]); // leaves the dial at 99
        assert_eq!(zeroes, 0);
        let zeroes = dial.apply(L![98]); // leaves the dial at 1
        assert_eq!(zeroes, 0);
        let zeroes = dial.apply(L![1]); // turns dial to 0
        assert_eq!(zeroes, 1);
        dial.apply(R![50]); // puts dial to 50
        let zeroes = dial.apply(R![50]);
        assert_eq!(zeroes, 1); // puts dial back to 0
        let zeroes = dial.apply(R![100]);
        assert_eq!(zeroes, 1);
        let zeroes = dial.apply(L![100]);
        assert_eq!(zeroes, 1);
    }

    macro_rules! spin {
        ($target:expr, $instruction:expr, $expected:expr, $position_after:expr) => {
            assert_eq!($target.apply($instruction), $expected);
            assert_eq!($target.position(), $position_after);
        };
    }

    #[test]
    fn test_part2_stepwise() {
        let mut dial = DialPosition::new(50);
        spin!(dial, L![68], 1, 82);
        spin!(dial, L![30], 0, 52);
        spin!(dial, R![48], 1, 0);
        spin!(dial, L![5], 0, 95);
        spin!(dial, R![60], 1, 55);
        spin!(dial, L![55], 1, 0);
        spin!(dial, L![1], 0, 99);
        spin!(dial, L![99], 1, 0);
        spin!(dial, R![14], 0, 14);
        spin!(dial, L![82], 1, 32);
    }
}
