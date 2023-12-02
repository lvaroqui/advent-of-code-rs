use common::DayResult;
use itertools::Itertools;

pub struct Solver;

const MAPPING: [([&str; 2], u32); 9] = [
    (["1", "one"], 1),
    (["2", "two"], 2),
    (["3", "three"], 3),
    (["4", "four"], 4),
    (["5", "five"], 5),
    (["6", "six"], 6),
    (["7", "seven"], 7),
    (["8", "eight"], 8),
    (["9", "nine"], 9),
];

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let res: u32 = input
            .split_whitespace()
            .map(|line| {
                let digits = line.chars().filter(|c| c.is_numeric()).collect_vec();
                let first = digits.first().unwrap().to_digit(10).unwrap();
                let last = digits.last().unwrap().to_digit(10).unwrap();
                first * 10 + last
            })
            .sum();
        DayResult::new(res)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let res: u32 = input
            .split_whitespace()
            .map(|line| {
                let mut line = line;
                let mut digits = vec![];

                for _ in 0..line.len() {
                    for (pats, val) in &MAPPING {
                        for pat in pats {
                            if line.starts_with(pat) {
                                digits.push(*val);
                                break;
                            }
                        }
                    }

                    line = &line[1..];
                }
                let first = digits.first().unwrap();
                let last = digits.last().unwrap();
                first * 10 + last
            })
            .sum();
        DayResult::new(res)
    }
}
