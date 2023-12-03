use std::collections::{HashMap, HashSet};

use common::DayResult;
use itertools::Itertools;

pub struct Solver;

enum Element {
    Digit(u8),
    Gear(u32),
    Part,
    None,
}
struct Map(Vec<Vec<Element>>);
impl Map {
    fn adjacent_iter(&self, x: usize, y: usize) -> impl Iterator<Item = &Element> {
        let x = x as isize;
        let y = y as isize;
        [
            // Line above
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            // Left
            (x - 1, y),
            //Right
            (x + 1, y),
            // Line bellow
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter(|&(x, y)| {
            x >= 0 && x < self.0[0].len() as isize && y >= 0 && y < self.0.len() as isize
        })
        .map(|(x, y)| &self.0[y as usize][x as usize])
    }
}

#[derive(Default)]
struct Number<Metadata> {
    value: u32,
    digit_index: u32,
    metadata: Metadata,
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve::<bool>(
            input,
            |has_neighbour, neighbour| {
                // Record if the current number has at list one adjacent part
                // number
                *has_neighbour =
                    *has_neighbour || matches!(neighbour, Element::Part | Element::Gear(_));
            },
            |numbers| {
                let res = numbers
                    .iter()
                    .filter(|v| v.metadata)
                    .map(|v| v.value)
                    .sum::<u32>();

                DayResult::new(res)
            },
        )
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve::<HashSet<u32>>(
            input,
            |gear_ids, neighbour| {
                // Record all gears adjacent to numbers
                if let Element::Gear(id) = neighbour {
                    gear_ids.insert(*id);
                }
            },
            |numbers| {
                // Construct map of gears with adjacent numbers
                let mut gears = HashMap::new();
                for n in numbers {
                    for gear_id in &n.metadata {
                        let gear = gears.entry(gear_id).or_insert_with(std::vec::Vec::new);
                        gear.push(n.value);
                    }
                }

                // Find gears adjacent to exactly 2 numbers, compute the sum of
                // theirs individual powers
                let res = gears
                    .values()
                    .filter(|v| v.len() == 2)
                    .map(|v| v.iter().product::<u32>())
                    .sum::<u32>();

                DayResult::new(res)
            },
        )
    }
}

fn solve<NumberMetadata: Default>(
    input: &str,
    inspect_number_neighbour: impl Fn(&mut NumberMetadata, &Element),
    compute_result: impl Fn(&Vec<Number<NumberMetadata>>) -> DayResult,
) -> DayResult {
    // Parse map
    let lines = input.split('\n');
    let mut gear_id = 0;
    let map = Map(lines
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    c if c.is_numeric() => Element::Digit(c.to_digit(10).unwrap() as u8),
                    '.' => Element::None,
                    '*' => {
                        gear_id += 1;
                        Element::Gear(gear_id)
                    }
                    _part => Element::Part,
                })
                .collect_vec()
        })
        .collect_vec());

    let mut numbers = vec![];

    for (y, line) in map.0.iter().enumerate() {
        let mut current_number: Option<Number<NumberMetadata>> = None;

        // Iterating in reverse allows easy parsing of the numbers (by reading
        // them from right to left and adding `digit * 10**digit_index` to the
        // number)
        let mut it = line.iter().enumerate().rev().peekable();
        while let Some((x, e)) = it.next() {
            // Current number must be consumed if:
            //  1. The current element is not a digit
            //  2. There is no more element to parse
            let mut consume_current_number = it.peek().is_none();

            if let Element::Digit(digit) = e {
                let c = current_number.get_or_insert(Number::default());
                c.value += 10_u32.pow(c.digit_index) * *digit as u32;
                c.digit_index += 1;

                for a in map.adjacent_iter(x, y) {
                    inspect_number_neighbour(&mut c.metadata, a);
                }
            } else {
                consume_current_number = true;
            }

            if consume_current_number {
                if let Some(number) = current_number.take() {
                    numbers.push(number)
                }
            }
        }
    }

    DayResult::new(compute_result(&numbers))
}
