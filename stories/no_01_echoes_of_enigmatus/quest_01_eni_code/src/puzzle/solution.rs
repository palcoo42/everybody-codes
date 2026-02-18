use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::formula::Formula;

pub struct Solution {
    formulas: Vec<Formula>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "Echoes of Enigmatus [No. 1] - Quest 1: EniCode"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/everybody_codes_e1_q01_p1.txt").unwrap_or_else(
                |err| panic!("Failed to fetch file ../input/everybody_codes_e1_q01_p1.txt [{err}]"),
            ),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        for line in lines {
            let formula = Formula::from_string(&line)?;
            self.formulas.push(formula);
        }
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let codes: Vec<_> = self.formulas.iter().map(|f| f.eni_code()).collect();
        let max = codes.iter().max().expect("Maximum not found, empty vector");
        Ok(max.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Not solved".into())
    }

    fn solve_part3(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Not solved".into())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { formulas: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "7424735351");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "Not solved");
    }

    #[test]
    fn test_solve_part3() {
        assert_eq!(get_puzzle().solve_part3().unwrap(), "Not solved");
    }
}
