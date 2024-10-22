use practical_rust::vec_exercises::rotate::{rotate, Direction};


#[test]
fn rotate_test() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = rotate(numbers, 5, Direction::Left);
    let expected_result = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5];
    assert_eq!(result, expected_result);
}


#[test]
fn rotate_test_2() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = rotate(numbers, 5, Direction::Right);
    let expected_result = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5];
    assert_eq!(result, expected_result);
}