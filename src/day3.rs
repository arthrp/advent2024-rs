use regex::Regex;

pub struct Day3Solver{
    data: String
}

impl Day3Solver {
    pub fn new(data: String) -> Self {
        Day3Solver { data }
    }

    pub fn solve(&self) -> i32 {
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let sum: i32 = re.find_iter(&self.data).map(|m| self.process_mul(m.as_str())).sum();

        sum
    }

    pub fn solve_additional(&self) -> i32 {
        let re = Regex::new(r"mul\(\d+,\d+\)|don\'t\(\)|do\(\)").unwrap();
        let mut sum = 0;
        let mut is_stopped = false;
        for m in re.find_iter(&self.data){
            if m.as_str() == "do()" { is_stopped = false }
            else if m.as_str() == "don't()" { is_stopped = true }
            else if !is_stopped { sum += self.process_mul(m.as_str()) }
        }
        sum
    }

    fn process_mul(&self, str: &str) -> i32 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let res: Vec<i32> = re.captures_iter(str).map(|c| {
            let (_, [first,second]) = c.extract();
            let prod = first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
            prod
        }).collect();

        res[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works(){
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let x = Day3Solver::new(data.to_string());
        let result = x.solve();
        assert_eq!(result, 161);
    }

    #[test]
    fn solve_additional_works(){
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let x = Day3Solver::new(data.to_string());
        let result = x.solve_additional();
        assert_eq!(result, 48);
    }
}