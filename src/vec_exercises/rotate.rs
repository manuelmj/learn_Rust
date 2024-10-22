
pub enum Direction {
    Left,
    Right
}

// fn rotate()

fn rotate_left(numbers: Vec<i32>, k: u32) -> Vec<i32> {
    let longitud = numbers.len(); 
    if longitud == 0 { return numbers; }
    let rotate_section:usize   = k as usize % longitud;
    if rotate_section == 0 { return numbers; }
    let (sub_vector_1, sub_vector_2) = numbers.split_at(rotate_section);
    [sub_vector_2, sub_vector_1].concat()

}

fn rotate_right(numbers: Vec<i32>, k: u32) -> Vec<i32> {

    let longitud = numbers.len();
    if longitud == 0 { return numbers; }
    let rotate_section: usize  = k as usize % longitud;
    if rotate_section == 0 { return numbers; }

    // let sub_vector_1 = numbers[rotate_section..].to_vec();
    // let sub_vector_2 = numbers[longitud - rotate_section..].to_vec();
    // let new_vector = sub_vector_1.into_iter().chain(sub_vector_2.into_iter()).collect();

    // new_vector
    let (sub_vector_1, sub_vector_2) = numbers.split_at(longitud - rotate_section);
    [sub_vector_2, sub_vector_1].concat()


}


pub fn rotate(numbers: Vec<i32>, k: u32, direction: Direction ) -> Vec<i32> {
    
    match direction {
        Direction::Left => rotate_left(numbers, k),
        Direction::Right => rotate_right(numbers, k),
    }

}