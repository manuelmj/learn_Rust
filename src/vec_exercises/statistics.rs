

use std::collections::HashMap;

// #[derive(Debug)]
#[derive(Debug, PartialEq)]
pub struct Statistics {
    pub mean: f64,
    pub median: f64,
    pub mode: Vec<i32>,
}

impl Statistics {
    fn new() -> Statistics {
        Statistics {
            mean: 0.0,
            median: 0.0,
            mode: vec![0]
        }
    }
}

struct MyVect(Vec<i32>);

impl MyVect{
    fn mean(&self) -> f64 {
        let sum = self.0.iter().sum::<i32>() as f64;
        sum as f64 / self.0.len() as f64
    }

    fn mode(&self) -> Vec<i32> {
        let mut frequency: HashMap<i32, i32> = HashMap::new();

        for &number in &self.0 {
            let count = frequency.entry(number).or_insert(0);
            *count += 1;
        }

        let max_count =  frequency.values().cloned().max().unwrap_or(0);
        let mut mode:Vec<i32> = frequency.into_iter()
                    .filter(|&(_, count)| count == max_count).map(|(num,_)| num)
                    .collect();
        mode.sort();
        mode  
    }

    fn median(&self) -> f64 {
        let mut sorted = self.0.clone();
        sorted.sort();
        let middle = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[middle - 1] + sorted[middle]) as f64 / 2.0
        } else {
            sorted[middle] as f64
            
        }
    }

}   


pub fn statistics(numbers: Vec<i32>) -> Statistics {
    let mut vec_statistics: Statistics = Statistics::new();
    let my_vec: MyVect = MyVect(numbers);
    
    vec_statistics.mean = my_vec.mean();
    vec_statistics.mode = my_vec.mode();
    vec_statistics.median = my_vec.median();
    vec_statistics
}