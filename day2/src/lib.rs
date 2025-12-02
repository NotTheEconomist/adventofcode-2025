use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;
use anyhow::Context;

/// is_valid tests to see if a given ID is valid.
/// invalid IDs are wholly duplicated, e.g. 123123, 11, 9292, etc.
///
/// ```
/// use day2::{is_valid};
/// assert!(is_valid(123));
/// assert!(!is_valid(11));
/// assert!(!is_valid(9292));
/// assert!(!is_valid(9876598765));
/// assert!(is_valid(179876214));
/// ```
pub fn is_valid(id: u64) -> bool {
    let s = format!("{}", id);
    // if the length is even then split the number in half and tell me if they're identical
    if s.len() % 2 == 0 {
        let (a, b) = s.split_at(s.len() / 2);
        return a != b;
    }
    true
}

/// is_valid2 tests to see if a given ID is valid in part2.
///
/// part2 valid IDs do not contain any possible groupings that are identical. Consider
/// 12341234 can be grouped in three ways (without truncation):
/// [1 2 3 4 1 2 3 4] is valid, because not all groups are the same
/// [12 34 12 34] is valid, because not all groups are the same
/// [1234 1234] is invalid, because all the groups are the same.
///
/// Because a grouping exists that is invalid, the ID is invalid.
///
/// ```
/// use day2::{is_valid2};
/// assert!(is_valid2(12345678));
/// assert!(!is_valid2(12341234));
/// assert!(!is_valid2(111111));
/// ```
pub fn is_valid2(id: u64) -> bool {
    let s = format!("{}", id);
    //
    // find all ways to split this string into even chunks
    let chars = s.chars().collect::<Vec<_>>();

    // This ugly bit of iterator rolls through all the valid sizes to chunk an ID into, e.g. 1234
    // in chunks of 2 is [12 34], in chunks of 1 is [1 2 3 4]
    let mut divisors = (1..=s.len()/2).filter(|&div| s.len() % div == 0);

    // If any of these divisors yield chunked groups that are all equal, then this ID is not valid.
    !divisors.any(|div| chars.chunks(div).all_equal())
}

#[derive(Debug, Clone)]
pub struct Input {
    ranges: Vec<RangeInclusive<u64>>
}

impl IntoIterator for Input
{
    type Item = u64;
    type IntoIter = std::iter::Flatten<std::vec::IntoIter<RangeInclusive<u64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter().flatten()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        for group in s.trim().split(',') {
            let (a, b) = group.split_once('-').context(format!("Failed to split range group {}", group))?;
            let a = a.parse().context(format!("Failed to parse {} as u64", a))?;
            let b = b.parse().context(format!("Failed to parse {} as u64", b))?;
            v.push(RangeInclusive::new(a, b))
        }
        Ok(Self { ranges: v })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parse_input() {
        let input = INPUT.parse::<Input>().unwrap();
        assert_eq!(input.into_iter().count(), 106);
    }
}
