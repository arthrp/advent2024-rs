pub struct Day1Solver{
    data: String
}

impl Day1Solver {
    pub fn new(data: String) -> Self {
        Day1Solver { data }
    }

    pub fn solve(&self) -> i32 {
        let lines : Vec<&str> = self.data.split('\n').collect();
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

        let mut acc = 0;

        for i in 0..arr1.len() {
            println!("{} {}",arr1[i], arr2[i]);
            let r = (arr1[i] - arr2[i]).abs();
            acc += r;
        }
        acc
    }
}