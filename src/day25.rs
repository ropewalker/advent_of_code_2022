use crate::day25::SnafuDigit::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl From<&SnafuDigit> for i64 {
    fn from(snafu_digit: &SnafuDigit) -> Self {
        match snafu_digit {
            Two => 2,
            One => 1,
            Zero => 0,
            Minus => -1,
            DoubleMinus => -2,
        }
    }
}

impl From<&SnafuDigit> for char {
    fn from(snafu_digit: &SnafuDigit) -> Self {
        match snafu_digit {
            Two => '2',
            One => '1',
            Zero => '0',
            Minus => '-',
            DoubleMinus => '=',
        }
    }
}

impl TryFrom<char> for SnafuDigit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Two),
            '1' => Ok(One),
            '0' => Ok(Zero),
            '-' => Ok(Minus),
            '=' => Ok(DoubleMinus),
            _ => Err(()),
        }
    }
}

impl TryFrom<i64> for SnafuDigit {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Two),
            1 => Ok(One),
            0 => Ok(Zero),
            -1 => Ok(Minus),
            -2 => Ok(DoubleMinus),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct SnafuNumber(Vec<SnafuDigit>);

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: String = self.into();
        write!(f, "{}", string)
    }
}

impl From<&SnafuNumber> for String {
    fn from(snafu: &SnafuNumber) -> Self {
        snafu
            .0
            .iter()
            .rev()
            .map(|snafu_digit| {
                let snafu_digit: char = snafu_digit.into();
                snafu_digit
            })
            .collect()
    }
}

impl TryFrom<&str> for SnafuNumber {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut digits = Vec::with_capacity(value.len());

        for digit in value.chars().rev() {
            let snafu_digit: SnafuDigit = digit.try_into()?;

            digits.push(snafu_digit);
        }

        Ok(SnafuNumber(digits))
    }
}

impl From<&SnafuNumber> for i64 {
    fn from(snafu_number: &SnafuNumber) -> Self {
        snafu_number
            .0
            .iter()
            .enumerate()
            .map(|(power, digit)| {
                let digit: i64 = digit.into();
                digit * 5i64.pow(power as u32)
            })
            .sum()
    }
}

impl From<i64> for SnafuNumber {
    fn from(value: i64) -> Self {
        let mut dividend = value;
        let mut snafu_digits = Vec::new();

        while dividend > 0 {
            snafu_digits.push(((dividend + 2) % 5 - 2).try_into().unwrap());
            dividend = (dividend + 2) / 5;
        }

        SnafuNumber(snafu_digits)
    }
}

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Vec<SnafuNumber> {
    input
        .lines()
        .map(|snafu_number| snafu_number.try_into().unwrap())
        .collect()
}

#[aoc(day25, part1)]
fn part1(snafu_numbers: &[SnafuNumber]) -> String {
    let sum = snafu_numbers
        .iter()
        .map(|snafu_number| {
            let value: i64 = snafu_number.into();
            value
        })
        .sum::<i64>();

    let snafu_number: SnafuNumber = sum.try_into().unwrap();
    snafu_number.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), "2=-1=0".to_string());
    }
}
