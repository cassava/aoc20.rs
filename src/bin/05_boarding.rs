/*!
# Day 5: Binary Boarding

You board your plane only to discover a new problem: you dropped your boarding
pass! You aren't sure which seat is yours, and all of the flight attendants are
busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby
boarding passes (your puzzle input); perhaps you can find your seat through
process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat
people. A seat might be specified like FBFBBFFRLR, where F means "front",
B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B; these specify exactly one of the
128 rows on the plane (numbered 0 through 127). Each letter tells you which
half of a region the given seat is in. Start with the whole list of rows; the
first letter indicates whether the seat is in the front (0 through 63) or the
back (64 through 127). The next letter indicates which half of that region the
seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.

The last three characters will be either L or R; these specify exactly one of
the 8 columns of seats on the plane (numbered 0 through 7). The same process as
above proceeds again, this time with only three steps. L means to keep the
lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.

So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the
column. In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

As a sanity check, look through your list of boarding passes. What is the
highest seat ID on a boarding pass?

## Part Two

Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding
pass in your list. However, there's a catch: some of the seats at the very
front and back of the plane don't exist on this aircraft, so they'll be missing
from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and
-1 from yours will be in your list.
*/

use aoc19;

use std::str::FromStr;
use thiserror::Error;

fn main() {
    let mut input = aoc19::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 5: {}", PUZZLE);

    let seating: Vec<SeatingPos> = input.to_str().lines().map(|x| x.parse().unwrap()).collect();
    let mut ids: Vec<usize> = seating.iter().map(|x| x.id()).collect();
    ids.sort();

    println!(":: Highest seat ID is {}", ids.last().unwrap());

    let mut found = false;
    let mut prev = ids.first().unwrap() - 1;
    for id in ids {
        if id - 1 != prev {
            found = true;
            println!(":: Your seat ID could be {}", id - 1);
        }
        prev = id;
    }
    if !found {
        println!(":: Didn't find a free seat for you :-(");
    }
}

pub struct SeatingPos {
    row: usize,
    col: usize,
}

impl SeatingPos {
    pub fn id(&self) -> usize {
        self.row * 8 + self.col
    }

    pub fn is_valid(&self) -> bool {
        self.row < 128 && self.col < 8
    }
}

impl FromStr for SeatingPos {
    type Err = SeatingPosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, col) = s.split_at(7);
        Ok(Self {
            row: binary_to_u8(row, 'B'),
            col: binary_to_u8(col, 'R'),
        })
    }
}

fn binary_to_u8(s: &str, one: char) -> usize {
    let s: String = s
        .chars()
        .map(|c| if c == one { '1' } else { '0' })
        .collect();
    usize::from_str_radix(&s, 2).unwrap()
}

#[derive(Error, Debug)]
pub enum SeatingPosError {
    #[error("Data invalid")]
    InvalidInput { value: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seating_pos() {
        struct Test {
            pub input: String,
            pub row: usize,
            pub col: usize,
            pub id: usize,
        };

        let tests = vec![
            Test {
                input: "FFFFFFFLLL".into(),
                row: 0,
                col: 0,
                id: 0,
            },
            Test {
                input: "FBFBBFFRLR".into(),
                row: 44,
                col: 5,
                id: 357,
            },
            Test {
                input: "BFFFBBFRRR".into(),
                row: 70,
                col: 7,
                id: 567,
            },
            Test {
                input: "FFFBBBFRRR".into(),
                row: 14,
                col: 7,
                id: 119,
            },
            Test {
                input: "BBFFBBFRLL".into(),
                row: 102,
                col: 4,
                id: 820,
            },
        ];

        for test in tests {
            let pos: SeatingPos = test.input.parse().unwrap();
            assert_eq!(pos.row, test.row, "row invalid");
            assert_eq!(pos.col, test.col, "col invalid");
            assert_eq!(pos.id(), test.id, "id invalid");
        }
    }
}

const PUZZLE: &'static str = "Binary Boarding";
const INPUT: &'static str = r"
BFFFBBFRLR
FBFFFBBLRL
BFFFFFBRRR
FBBBFFFLRR
FFBFBFBRLL
FBBFFBBLRR
FBBFFFFRLL
BFFFBBFRLL
BBFBFBFRLL
BFFFFFBRLL
FFBFFFBRRR
BBFFBFFRLR
BBFFBBFLLL
FBFBFBFLLL
BBFFFFFLLR
FFBBBFFRLL
FFBFFBFRRR
BFFFBBFLRL
BFBBBBFLRR
BFBFFBFLLL
FFBFFBBLRL
FFBFBFFRLR
FFBFBBBLLR
FFFBBFBLLR
FFFBBBFLRL
FBFBBBFLLL
FBBFFFBLLR
BFFFBBBRLL
FFBBFBBRRL
FBFFBBBLLR
FBFBBFFRRL
FBBBBFFLLL
FFBFBFFLLR
BFFFFBFLLR
FBBBFBFLRR
FBFFFFFRRR
FFBFBBFRRL
BFFFBBBLLR
FBBFFBFRLL
BFFFFBBRLL
FBBBFFFRRL
BBFBFFFLLR
FBBFBBFRLR
FFFBFFBRRR
BFBBFBBLRL
FBBBFFBLRR
BBFFBBFLRR
FFBBFBBLLL
BFFBBBBRRL
FBBFBFBRLL
BFFFFFFRRL
BBFBFBFLLL
FBFFBFFLLR
FBFBFBFRLR
BFBFBBBRLR
BBFFBBFLLR
BFFBFBFRRR
BFBBFBFLLL
FBFBBFBRLR
FBBFBFBLRL
FBFFBFFLRR
BFFFFFFRRR
BFFFFBFRLL
FBFBFFFLLR
BFFFFBFLRL
BBFFFFFRLL
FBFBFFFLRR
BFFBFBBLRL
BFBFBBFLLR
BFBFBBBLRL
BFFBFFFRRL
BFFFFFFLLL
FFBBBBBLLR
FBBBFFFLLL
BFFFBBFRRL
FFFBFBBRLL
FFFBFFBRRL
BFBFBFBRRR
BBFFFBFLLR
BFFFBFFLLL
FBFBBFBLRR
FFBBBBFLRR
BFFBBBFLRR
BBFFBBBLLR
FFFBFBBRRL
BBFFFBBLRR
FFBFFFBRRL
FFBFFFBRLR
BFBFBFFRLL
FBFFFBFLRL
BBFFFFFRLR
BFBBFFBRLR
FBFBFBBRRR
BFBFFFFLLR
FFBFBFBLRR
FFFBBBBRRL
FBFFBBFRRR
BFFBBBBLRR
BFFBFFFLLR
BFBFFBFLLR
FBBBBBBRLR
BFBFFFFLRL
BBFFBFFLLL
FBBBBBBLLR
FFBBBBBRRL
BFBFFFFLLL
FBFBBBFRLL
BFFBBBFLRL
FBBFBBBLLL
FBFBBFBRRR
FFBBFFFRLL
BBFFBBBLRL
FBBFFFBRLL
BBFBFFBRLR
FBBBFFBLLR
BFBFFFBRRL
FBFFBBBRLL
FBFBBBBRLL
BFFFBFFRLR
FFBBFBFRLR
BFBFBFFLRR
FFFBBFFLRR
BFBFBBBLLL
BFBFBFBRLR
BFFFFBFLRR
FFBBBFBRRR
BFFFFBBLLR
BBFFBFBLRR
BFBFFFBRRR
BFFFFBBRLR
FBBBBBFRRR
FBBFBBFLLR
FFBBFBBRLL
BFBBFFBLRR
FBBBFFFLLR
BFFFBFFLRL
FBBBBFBLLR
BFBBFBFLRR
BFBBBFFLLL
BFFBFBFRLL
BFBBFBFRRL
BFFBFFBLRR
FFBFFFBLLL
FFFBFBFLRL
FBBBFBBLRR
BFFFBBBLRR
FBFFFFFRLR
BBFFBBBLRR
BFBFFBBLRR
FBFFBBBRRR
FBFFBFFLLL
BFBBBBFRLL
FBBBBBBRRL
BFFBBBBLRL
FFBFBBFLRR
FFBBBFBLLL
FBBFFFFRLR
BBFBFBFRRR
FBFBFFBLRL
FFBBFBBLRL
FBFBBFFLLL
BFFFBFBLRR
FBFBFFBLLR
FBBBBBFLRL
FFBFBFFRLL
BBFFBBFRRR
FFFBBFFLLR
BFBBBFBLRR
FFBBFFBLRL
BFFBBBFRRL
BFBBBBFRRL
BBFFBFFRRL
FFBFFBBRRR
BFFBFBFLRR
FBFBFFBRRR
FBBFBBFLRL
FBFFFFBLRL
FBFFFBBLLL
FFBFFBFRLR
FBFFFBBRRL
BFBBBFBLLL
BFBFBFFLRL
BFBFFBFRRR
BBFFBBBRLL
BBFFFBFLRL
BBFFBFFRLL
FBBFFBFLLL
FBBBBFFLLR
FFBFFFFLLR
FFBBBBFLLR
BFBBBBBRRR
FFBBFBFRRR
FFBFFBFLRL
FBFBBFFRRR
BFBBBFFRRR
FBBFBBFRLL
FBBFBBBRLR
BFFFFFBRLR
BFFBFBBRLL
FBFFFBBLRR
FFBFFFFRLL
FBBBBFBRRL
FBBBBFBRLR
BFBBBBFLLL
FBBBBBFRLL
FBFBFBFLRL
FFBFBBFLLL
FBBBBFFRRL
BFFBFBFLLL
FFFBBBFLLL
FBFBBBBLLL
BFBBBBFRLR
BBFBFFFRRR
FBBBBBBLRL
BFFFBFBLRL
FFBBFBFLLR
BFBBBFBLRL
FBBBBBBRLL
FFBBFFFRRR
BFFFFBBRRR
FBFFBBBLLL
BBFBFFFLRL
FFBFFFBLRR
FFBBBFBLLR
BFFBBFFRRR
FBBFBFFLRL
FFBBBBBLRR
FBBBFBBLLL
BBFBFBFRLR
FBFBFBBLRR
FBBBFBBRLR
FBFBFBBLLR
FFFBBFBRRR
FBBBFFBRLL
FFBBBBFLRL
BBFFBFFRRR
FBFBFBFLLR
FBBBFBFRLR
BFFBBBFRRR
FFFBFBBLLR
BBFBFBBLRR
FBFFBBFRLR
FFBBBFFLLR
FFBBFBFLRL
BFFBFFFLLL
BFBFFBFRRL
FBFBBBBRRL
BFBBBBFLLR
FBBFFBBRLL
BFFBBFBRLR
BFBFFFBLRR
FFBBFFBLRR
FBBFFFFRRL
BFBBFBBLLR
FBFFFBFRLR
BFBBBBFLRL
BFBBBBBRLL
FFBBBBFRLR
BBFBFFBLLR
FBBFFFBLRR
FBFFFFFLLL
FBBFFBFRLR
FBFFBBBRRL
FFBFBFBRRL
FFBBBBFLLL
FFBFBBBRLR
FBBFBFBRLR
BBFFFBBLLR
FBBFFBBLLL
FFBBFFFLLL
FBBBBBFLRR
FFBBFFFLRR
BFFBFFBLLR
FBFFBFBRRR
FFFBFFBRLR
FBFFFFFLRR
BFFFBFBRLL
FBFBBFBRRL
FFBFBBBLRL
BFFFBBFLLR
FBBFFBBRRR
BFBBFBFRLL
FBFFFFBRLL
BBFFFFBLRR
BFFBFBFLLR
FFBFBBBRRL
FBBFFFBRLR
FFBBBBFRRR
BFBBFFBLLR
BFBFFFBLLL
BFFFBBBRRR
FBFBBBFRRR
BFBFFBBRRL
FBFBBFBLLR
BFFFBBBRLR
BFFFFBFLLL
BFFBFFBRLR
FBBFBFBRRR
FFBBFBBRRR
FBFFFBBLLR
BFBBFBBRLR
FFBBBBBRLR
FBFFBBFRRL
BFFBBBFLLR
FBBBFFBLLL
BFBFBFFRRR
BFBFBBFRRR
FBBFFBBLLR
BFFFFFFLLR
FBBBBFFRLR
FBBBBBBLLL
BFBBBBFRRR
BFBBBFFRLR
FFBFFBBRLL
FFBBFBFLLL
BFBFFFBLRL
FFFBBBFLRR
FBBFBFFRRL
FFFBFBFLLL
BFFBBFBLLL
BBFFFFBLLL
BFBBBBBRRL
FBBFBFBLRR
FBFFBFBRLL
BBFBFBBLLL
FFBBBFBLRR
FBBBFFFRRR
BFFBBBBLLL
FBFBFBFRRR
BFFFFFBLLL
BFBFBFBLLR
BBFFFFBRRL
FBBBFBFLLL
FFBFFFFLLL
FBFBFFBLLL
FBBBBBBRRR
BFBFFBFLRR
FFBFBFBLLL
FFBFFBFRRL
BFBFBBFRRL
FFBBFBBLLR
FBBBBBFLLR
BFFBFFFRLL
BBFFFBBLLL
FBFFFFFLLR
FFBBBFFRRR
FBFFFBFRRR
FFFBBBBLRL
FBFBFBBRRL
FBBFBFBLLL
BFBFFFFLRR
BBFFBFBLRL
FFFBFBFRLR
BFFBBFFLRL
FFFBBFFRRL
BFBFBFBRRL
FFBBBFFLLL
BBFFFBFLRR
FFFBBBBLLR
FFBBBFFRLR
FFBFBBFLRL
FFBBBFBLRL
BFBFFFFRRL
FBFBBFFLRR
FBBBBBBLRR
FBFBBBBRRR
FFBFBFFLRR
FBBFBFFRLR
BFFBBBBRRR
BFFFBFBLLL
BFFBBFFLLL
BFFFFBBLRL
FFBBBBBLLL
FBBBBFFRRR
FFBBFFBRRL
BBFBBFFLLL
FFFBBFBRRL
BFBFFFFRLR
FBFFBFBLLL
FFFBBFFLRL
BBFFBFFLLR
FBBFBFFLRR
FBFFBBBRLR
FBFFFFFRRL
FFFBBFBRLL
BFFBFFFLRL
BFFBBFBLRR
FFBFBFBLRL
FBBFBFBLLR
BBFFFFBLLR
BFBFBFBLRL
FFBBBFBRRL
BBFFBFBLLR
BBFFFFBRRR
FFBFBFBRLR
BBFBFFFRLR
FBBBFBFRLL
FFBBFBFRRL
BBFBFBBLLR
FFBBFFFLRL
FFBFFBBLLL
FBFBBFFLLR
FFBFFFFLRR
BFBBFFBRLL
BFBBFBBLLL
BBFBFFFRRL
FBBBFBBRRL
BBFBFBBRLR
BBFBFFBRRL
FBFBBBFLRL
BFBFBFFRRL
FFBBBBFRRL
BFFFBBFRRR
FFFBBBBRLL
FBBBFFBRLR
FBBFBBBRLL
BFBFFBBLRL
FFFBBBFLLR
BFBBFBBRRR
FFFBFBBLLL
FFFBBBBRLR
FBFFBFBLLR
FBFBFBBRLR
FFBBFFBRLR
BBFFFFFLLL
FFFBFFBRLL
FFBBBBBRLL
FBBFBFFLLR
BBFFFBBLRL
FBBBFBBLLR
BFFFFFFLRL
FFFBFBFRRR
BFFFFFBLRL
BFBBFBFLRL
BFBBFFBRRL
FBFBBFFRLL
FFBFBFFRRL
BFFFBBFLLL
BBFFBFBRLR
BFFBFBBLLR
FBFBBBBLRL
FBFFBFFRRR
FBFFFBBRRR
FBBFFBBLRL
FBFBBBBRLR
BFBBFFFRRR
BFFBBFBRRL
FBBFBFFRLL
FFBBFFFRLR
FFFBFBFLRR
FFBBBBFRLL
FBBFBBBLRL
BBFFBBFRLR
BBFFFBFLLL
BFFFFBFRRL
BFFBBFBLLR
FFBFFFBLLR
FFFBFBBLRR
BFBFFBFLRL
BFFBBFBRLL
FBFFBBFRLL
FFBBFBFLRR
BBFFBFBRLL
FBBBFFBRRL
FBFBBBFLLR
FFBBFFFLLR
FBFFBBFLRR
BFBFBFBLLL
BBFBFFBLLL
BFFFBBFLRR
BFBBBFBRLL
BFBFBBFRLL
BBFBFFFLLL
FBFFFFBLRR
BFFFBFBLLR
FFBBFFBLLL
FBBBFFFLRL
BFFBFFBRRL
FBBFBFBRRL
FBBBBBFRLR
FBBBBFFLRL
FBBBFBFRRL
BFFFBBBRRL
FBBFBFFRRR
BFFFFBFRLR
BBFBFFBLRR
BFBBBBBRLR
FBFFFBBRLR
FBFFBBFLRL
FBFFFFBLLR
BFFBFBFRRL
FFBFBBBLLL
FBBFFBFLRR
BFBFBFBLRR
FFFBFBFRRL
FFBBBBBLRL
BFFBFFFRRR
FFBFBFBLLR
FFBFFBBRRL
BFFBFFBRLL
BBFFBFBLLL
BFBBFBBRRL
FFBFFBBRLR
BFFFBFFRRR
BFBBFFBLLL
FBFFFBFLLR
BFBFFFBLLR
FFBBBFFLRL
FFBFBBFRRR
BFBBFFFRLR
BFFFBFFLLR
FBBFBBFRRL
FBBFFBFLRL
BFBBFBFLLR
FFBBBFBRLR
FBFBBFFLRL
FBFBFBFRRL
FBBBFBFLRL
FBFBBFFRLR
FFBFFBFLRR
BFFBFBBRRL
BFFBBBBRLL
BFBFBBBRRL
BFFFFBBRRL
BFFFFBBLLL
FBFFFBFLRR
BFFFBBBLRL
BBFFBBFLRL
FFBBFBFRLL
FBBBFFFRLR
FBFFBFBRRL
BFBBBBBLLL
BBFFBFBRRR
FBBBFFBLRL
BFBBFFFLLL
BBFFFFFRRR
FFFBFFBLRR
FBBBFBFLLR
BBFFFBBRRR
FBFFFBFLLL
BBFFFBFRLR
BFFBFBFRLR
BFBBBFFRRL
BFBFBBBRLL
BFFBBFBLRL
FFBFBBFLLR
FBBFFFBLLL
BBFBFBFLRL
FFFBFBBRRR
BBFFFFBRLL
BFFBFFBLLL
BFFFBBBLLL
FFFBBFBLRR
FBBFBBBLRR
FBFFFBBRLL
FFBFFFBLRL
BBFFFBFRLL
BFBFBFFRLR
FBFBFBBRLL
FBBBFFBRRR
FBFBFFFRLR
BFBBBFBLLR
FBFFFBFRRL
BFBBFFFLRL
BFFBBFBRRR
BBFBFFFRLL
BFBBBFFLLR
FBFFBBBLRR
BFFBBBFRLL
BFBBFFFRRL
FFFBBBBLRR
BBFFBBBRLR
FBFBBBBLLR
BFFFBFBRRR
FBFBFFFLLL
FFFBBBFRRR
FFBFFBFLLL
BBFFBBBRRR
FFBFBBBRLL
FBBBBFFLRR
FBBFFFBRRL
FBBBFBBRRR
FBFBBFBLRL
BBFFFBBRLL
FBFBFFBLRR
FFBFFFFRRL
FBBBBFBLLL
FFFBBFFRRR
BFBFBBFLRL
BFBFBBBRRR
FBFFBBFLLR
FBFBFBBLLL
BFBFBBFRLR
FFBBBFBRLL
BBFBFBFLLR
BFFFBFFRRL
FFFBBBFRLL
FBFFBFFLRL
BFFFFFBLLR
FFFBBFBRLR
BBFBFFBLRL
BFFBBFFRLL
BFBBFFFLRR
BFBBBBBLLR
BFFBBFFLLR
FFBFBFBRRR
FFBBFFBLLR
FBBBFBBRLL
BFBBFFBLRL
FBBFBBBRRL
FBBBBFBLRL
BFBBBFFLRL
BFFFFFBLRR
FBFBFFFRLL
FBFBFFBRRL
FBFFBFFRLL
BFBFBFBRLL
BFFBFBFLRL
BFBBFFBRRR
BBFFFFFLRL
FBBBFBBLRL
BFBFFBBRLL
BBFFFBBRLR
BFBFFBFRLL
FFBFFFBRLL
BFFBBBBLLR
FBFFFFFLRL
BFBBBFFRLL
BFFBFBBLLL
BBFFBBBLLL
BFFBBFFLRR
BFFBFFFRLR
FBFBFFFRRL
FBBFFBFRRL
FBBFBBBRRR
BFFFFFBRRL
BFBBFBFRRR
BBFFBFFLRL
FBFBBFBRLL
FBFBFBFLRR
BBFFBBFRLL
FFBBBFFRRL
BFFBBFFRRL
BFFBFBBRRR
BBFFFFFRRL
FFBBFBBRLR
BBFBFBBRRR
BFBFFFFRLL
BFFBBFFRLR
FFFBBFBLRL
BBFFFFFLRR
BFBFBBFLLL
BFBBBBBLRR
FFFBFBFLLR
FBFFFFBLLL
BFFBBBFLLL
BFFBBBBRLR
BFFFFFFLRR
FFFBFBBLRL
BFFFFBFRRR
FBFFBBFLLL
FBFFFBFRLL
BBFBFFBRLL
BBFFFFBLRL
BBFFBFFLRR
FBBBBFBRRR
FBBFFFFLLL
FFBBBBBRRR
BFBFFBBLLR
FBFFFFFRLL
FFBFBFFRRR
FFBFFBFLLR
BFBFBBFLRR
FBFBBBBLRR
FBBFFFFLRR
FFBFBBBLRR
FBFFFFBRLR
FBFBBBFLRR
BFBFFFFRRR
FBBFFFFLLR
BBFBFBBLRL
FFBBFFBRRR
FFBFFBBLLR
BFBFBFFLLR
FBBFBBFLRR
FBBFFFBLRL
BFFBFBBLRR
FFBBFFFRRL
FBFFBFBRLR
FBFFBFBLRL
FFBFBBFRLR
FFBFFFFLRL
BBFFFFBRLR
FBFFBFBLRR
BFFBFFBLRL
BFBFFBBRRR
FBFFFFBRRL
FBFBFBBLRL
FFFBBBBRRR
BBFFBBFRRL
FBBFFBBRLR
BFFFFFFRLR
FBBFBBFRRR
FBFBBBFRLR
BFBFFFBRLR
FBBFFBFRRR
FFBBFBBLRR
FFFBBFFRLR
FFFBFBBRLR
FBBFFFFLRL
FBFBFFBRLL
BFBBFFFLLR
FBBFBBBLLR
BFFFFFFRLL
BBFFBBBRRL
FBFBBFBLLL
BBFBFBFLRR
FFBFBFFLRL
FFBFBBFRLL
FBBBFFFRLL
BBFFBFBRRL
FBBBFBFRRR
FFBFBFFLLL
FBFFFFBRRR
BFBFFBBLLL
FFFBFBFRLL
FBBFBBFLLL
BBFFFBFRRL
FFFBBFBLLL
BBFBFBBRLL
BFBBBFBRRR
BFFFFBBLRR
BFBFFFBRLL
BFFFBFFLRR
BFFBBBFRLR
FBBFFFFRRR
FBFFBBBLRL
FFBBBFFLRR
FFBFBBBRRR
BFBBFFFRLL
BFBFBBBLLR
FFFBBFFRLL
FBFBFBFRLL
FBBBBFFRLL
FFFBBBFRLR
BFBFBBBLRR
FBFFBFFRRL
FFBFFBFRLL
FBBFFFBRRR
FBBFFBBRRL
BBFFFBFRRR
FFFBBBFRRL
FFFBBFFLLL
BFBBFBFRLR
FBBBBBFLLL
BFBFBFFLLL
FBFBFFFRRR
FBFFBFFRLR
BFBBFBBLRR
FFBBFFBRLL
BFBBFBBRLL
FBBBBBFRRL
BFFFBFBRLR
BFBBBBBLRL
BFFBFFBRRR
BBFBFBFRRL
BFBFFBFRLR
BFFBFFFLRR
FBBBBFBRLL
BBFBFFBRRR
BFFFBFBRRL
BFFBFBBRLR
FBBBBFBLRR
BFBFFBBRLR
BFBBBFBRLR
BBFFFBBRRL
FBBFBFFLLL
FFBFFFFRLR
FBFBFFBRLR
BFFFBFFRLL
FBFBBBFRRL
FBFBFFFLRL
BBFBFFFLRR
FFBFFFFRRR
BFBBBFBRRL
FFFBBBBLLL
BBFBFBBRRL
FFBFFBBLRR
FBBFFBFLLR
";
