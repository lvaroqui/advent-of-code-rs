use chumsky::prelude::*;
use common::DayResult;
pub struct Solver;

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}
impl Set {
    fn possible_with(&self, bag: &Set) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let lines = input.split('\n');
        let parser = parser();

        let bag = Set {
            red: 12,
            green: 13,
            blue: 14,
        };

        let res = lines
            .map(|line| parser.parse(line).unwrap())
            .filter_map(|game| {
                game.sets
                    .iter()
                    .all(|s| s.possible_with(&bag))
                    .then_some(game.id)
            })
            .sum::<u32>();

        DayResult::new(res)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let lines = input.split('\n');
        let parser = parser();

        let res = lines
            .map(|line| parser.parse(line).unwrap())
            .map(|game| {
                game.sets
                    .into_iter()
                    .reduce(|acc, val| Set {
                        red: acc.red.max(val.red),
                        green: acc.green.max(val.green),
                        blue: acc.blue.max(val.blue),
                    })
                    .unwrap()
                    .power()
            })
            .sum::<u32>();

        DayResult::new(res)
    }
}

fn parser() -> impl Parser<char, Game, Error = Simple<char>> {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let set = text::int(10)
        .padded()
        .then(
            just("red")
                .map(|_| Color::Red)
                .or(just("green").map(|_| Color::Green))
                .or(just("blue").map(|_| Color::Blue)),
        )
        .separated_by(just(','))
        .map(|colors| {
            let mut set = Set::default();
            for (value, color) in colors {
                let value = value.parse::<u32>().unwrap();
                match color {
                    Color::Red => set.red += value,
                    Color::Green => set.green += value,
                    Color::Blue => set.blue += value,
                }
            }

            set
        });

    just("Game ")
        .ignore_then(text::int(10))
        .then_ignore(just(':'))
        .then(set.separated_by(just(';')))
        .map(|(id, sets)| Game {
            id: id.parse().unwrap(),
            sets,
        })
}
