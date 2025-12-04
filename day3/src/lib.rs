use anyhow::{Context, anyhow};
use std::str::FromStr;

/// Battery banks produce joltage.
///
/// ```
/// use day3::BatteryBank;
///
/// let bank: BatteryBank<2, u32> =  "987654321111111".parse().unwrap();
/// assert_eq!(bank.joltage(), 98);
/// ```
pub struct BatteryBank<const N: usize, T>(Vec<T>);

impl<const N: usize, T> FromStr for BatteryBank<N, T>
where
    T: From<u32>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < N {
            return Err(anyhow!(
                "Can't construct a BatteryBank of size {} with a string of length {}",
                N,
                s.len()
            ));
        }
        let bank = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(Into::<T>::into)
                    .context(format!("Failed to parse battery {}", s))
            })
            .collect::<Result<Vec<T>, _>>()?;
        Ok(BatteryBank(bank))
    }
}

impl<const N: usize, T> BatteryBank<N, T>
where
    T: Ord,
    T: SelfComposable,
    T: Into<u64>,
    T: Default,
    T: std::fmt::Debug,
{
    /// The joltage for any given N should be solvable recursively by finding the largest value
    /// in the available search space.
    /// If our bank is length 20 and we are looking for a 12-digit result, then the first digit
    /// is the first occurrence of the largest single digit within the first 9 indices (leaving
    /// room for the other 11 digits thereafter).
    ///
    /// There should be optimizations possible here merely by noticing when the remaining digits
    /// match exactly with the length of the remaining joltage, (e.g. if we know the first 3
    /// digits are 987 and there are exactly 9 elements left to search, then it must be exactly
    /// those 9 elements in that order)
    ///
    /// ```
    /// use day3::BatteryBank;
    /// let bank: BatteryBank<12, u64> = "987654321111111".parse().unwrap();
    /// assert_eq!(bank.joltage(), 987654321111);
    /// ```
    /// ```
    /// use day3::BatteryBank;
    /// let bank: BatteryBank<12, u64> = "811111111111119".parse().unwrap();
    /// assert_eq!(bank.joltage(), 811111111119);
    /// ```
    /// ```
    /// use day3::BatteryBank;
    /// let bank: BatteryBank<12, u64> = "234234234234278".parse().unwrap();
    /// assert_eq!(bank.joltage(), 434234234278);
    /// ```
    pub fn joltage(&self) -> u64 {
        let bank_length = self.0.len();
        let mut indices: Vec<usize> = Vec::with_capacity(N);
        while indices.len() < N {
            // The number of digits we still need to find
            let missing_digits = N - indices.len();
            // The next character cannot be closer to the front of the bank than this.
            let search_space_start = indices.last().copied().map(|idx| idx + 1).unwrap_or(0);
            // The next character cannot be further from the front of the bank than this.
            let search_space_end = bank_length - missing_digits + 1;

            // If the search space is the whole rest of the vector AND we need to use all those
            // characters to fill the result bank, then we can early out and take it all.
            if bank_length - search_space_start == missing_digits {
                indices.extend(search_space_start..bank_length + 1);
                break;
            }

            let next_idx = self.0[search_space_start..search_space_end]
                .iter()
                .zip(search_space_start..)
                .reduce(
                    |max @ (max_val, _max_idx), next @ (next_val, _next_idx)| {
                        if next_val > max_val { next } else { max }
                    },
                )
                .map(|(_, idx)| idx)
                .expect("bank cannot be empty");
            indices.push(next_idx);
        }

        let digits = indices
            .into_iter()
            .filter_map(|idx| self.0.get(idx))
            .cloned()
            .collect::<Vec<_>>();
        SelfComposable::try_compose(digits)
            .expect("Cannot fail to compose banks")
            .into()
    }
}

/// Composable objects can be combined in a useful way, producing
/// <Self as Composable<T>>::Output
pub trait Composable<T> {
    type Output;

    fn compose_with(self, other: T) -> Self::Output;
}

impl Composable<u32> for u32 {
    type Output = u32;

    fn compose_with(self, other: u32) -> u32 {
        self * 10 + other
    }
}

impl Composable<u64> for u64 {
    type Output = u64;

    fn compose_with(self, other: u64) -> u64 {
        self * 10 + other
    }
}

pub trait SelfComposable: Composable<Self, Output = Self> + Sized + Clone {
    fn try_compose<I>(xs: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        xs.into_iter().reduce(|acc, x| acc.compose_with(x))
    }
}
impl<T> SelfComposable for T where T: Composable<T, Output = T> + Clone + Sized {}
