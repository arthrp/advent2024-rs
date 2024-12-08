pub struct Day2Solver {
    data: String
}

impl Day2Solver {
    pub fn new(data: String) -> Self {
        Day2Solver { data }
    }

    pub fn solve(&self) -> i32 {
        let lines = self.data.split('\n');

        let mut safe_reports = 0;
        for line in lines {
            if line.len() == 0 { continue; } //last line is empty
            let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            let inc = if self.is_safe(nums) { 1 } else { 0 };
            safe_reports += inc;
        }
        safe_reports
    }

    fn is_safe(&self, report: Vec<i32>) -> bool {
        let is_asc = report[0] < report[1];

        for i in 0..report.len()-1 {
            if is_asc && (report[i+1] - report[i] > 3 || report[i+1] - report[i] < 1){
                return false;
            }
            if !is_asc && (report[i] - report[i+1] > 3 || report[i] - report[i+1] < 1){
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works(){
        let data = "7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n\
        \n\
        ";
        let d = Day2Solver::new(data.to_string());
        let result = d.solve();
        assert_eq!(result, 2);
    }
}