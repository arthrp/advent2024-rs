pub struct Day1Solver{
    data: String
}

impl Day1Solver {
    pub fn new(data: String) -> Self {
        Day1Solver { data }
    }

    pub fn solve(&self) -> i32 {
        let (arr1, arr2) = self.parse();

        let mut acc = 0;

        for i in 0..arr1.len() {
            let r = (arr1[i] - arr2[i]).abs();
            acc += r;
        }
        acc
    }

    pub fn solve_additional(&self) -> i32 {
        let (arr1, arr2) = self.parse();

        let mut sim_score = 0;

        for el in arr1 {
            let count = arr2.iter().filter(|&&x| x == el ).count();
            let inc: i32 = el * (count as i32);
            sim_score += inc;
        }
        sim_score
    }

    fn parse(&self) -> (Vec<i32>, Vec<i32>) {
        let lines  = self.data.split('\n');
        let mut arr1: Vec<i32> = [].to_vec();
        let mut arr2: Vec<i32> = [].to_vec();

        for line in lines {
            if line.len() == 0 { continue; }
            let x: Vec<&str> = line.split_whitespace().collect();
            arr1.push(x[0].parse().unwrap());
            arr2.push(x[1].parse().unwrap());
        }

        arr1.sort();
        arr2.sort();

        (arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "3   4\n\
    4   3\n\
    2   5\n\
    1   3\n\
    3   9\n\
    3   3\n";

    #[test]
    fn solve_works(){
        let x = Day1Solver::new(DATA.to_string());
        let res = x.solve();
        assert_eq!(res, 11);
    }

    #[test]
    fn solve_additional_works(){
        let d = Day1Solver::new(DATA.to_string());
        let res = d.solve_additional();
        assert_eq!(res, 31);
    }
}