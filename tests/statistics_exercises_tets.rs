use practical_rust::vec_exercises::statistics::{statistics, Statistics};



#[test]
fn statistics_test() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10,1];
    let result = statistics(numbers);
    let expected_result = Statistics {
        mean: 5.090909090909091,
        median: 5.0,
        mode: vec![1]
    };
    assert_eq!(result, expected_result);

}



#[test]
fn statistics_test_2() {

    let numbers = vec![1, 2, 3, 4, 5,3, 6, 7, 8, 9, 10,1];
    let result = statistics(numbers);
    let expected_result = Statistics {
        mean: 4.916666666666667,
        median: 4.5,
        mode: vec![1,3]
    };
    assert_eq!(result, expected_result);
}

