use chumsky::prelude::*;
use common::DayResult;

pub struct Solver;

#[derive(Debug)]
struct RangeMapping {
    destination: u64,
    source: u64,
    size: u64,
}

impl RangeMapping {
    fn map(&self, element: u64) -> Option<u64> {
        if element >= self.source && element < self.source + self.size {
            Some(self.destination + (element - self.source))
        } else {
            None
        }
    }

    fn reverse_map(&self, element: u64) -> Option<u64> {
        if element >= self.destination && element < self.destination + self.size {
            Some(self.source + (element - self.destination))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Mapping {
    _from: String,
    _to: String,
    ranges: Vec<RangeMapping>,
}
impl Mapping {
    fn map(&self, element: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| r.map(element))
            .unwrap_or(element)
    }

    fn reverse_map(&self, element: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| r.reverse_map(element))
            .unwrap_or(element)
    }
}

#[derive(Debug)]
struct Input {
    individual_seeds: Vec<u64>,
    range_seeds: Vec<std::ops::Range<u64>>,
    mappings: Vec<Mapping>,
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let input = parser().parse(input).unwrap();

        let mut elements = input.individual_seeds.clone();

        for mapping in input.mappings {
            for element in &mut elements {
                *element = mapping.map(*element);
            }
        }

        DayResult::new(elements.iter().min().unwrap())
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let input = parser().parse(input).unwrap();

        for location in 0..u64::MAX {
            let mut element = location;

            for mapping in input.mappings.iter().rev() {
                element = mapping.reverse_map(element);
            }
            if input.range_seeds.iter().any(|r| r.contains(&element)) {
                return DayResult::new(location);
            }
        }

        unreachable!("(until a long time)")
    }
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<u64>().unwrap());

    let seeds = just("seeds: ").ignore_then(
        number.separated_by(text::whitespace()).rewind().then(
            (number
                .separated_by(text::whitespace())
                .exactly(2)
                .map(|n| n[0]..n[0] + n[1]))
            .separated_by(text::whitespace()),
        ),
    );

    let range = number
        .separated_by(text::whitespace())
        .exactly(3)
        .map(|numbers| RangeMapping {
            destination: numbers[0],
            source: numbers[1],
            size: numbers[2],
        });

    let mapping = text::ident()
        .then_ignore(just("-to-"))
        .then(text::ident())
        .then_ignore(just(" map:").then(text::newline()))
        .then(range.separated_by(text::newline()))
        .map(|((from, to), ranges)| Mapping {
            _from: from,
            _to: to,
            ranges,
        });

    seeds
        .then_ignore(text::whitespace())
        .then(mapping.padded().repeated())
        .map(|((individual_seeds, range_seeds), mappings)| Input {
            individual_seeds,
            range_seeds,
            mappings,
        })
        .then_ignore(end())
}
