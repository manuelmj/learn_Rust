use std::collections::HashSet;

pub fn duplicate_items(numbers: Vec<i32>) -> Vec<i32> {
    let unique_number:  HashSet<i32> = numbers.into_iter().collect();
    let mut result:Vec<i32> = unique_number.into_iter().collect(); 
    result.sort(); 
    result 
}