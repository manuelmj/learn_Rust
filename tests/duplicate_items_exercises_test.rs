use practical_rust::vec_exercises::duplicate_items::duplicate_items;


#[test]
fn duplicate_items_test() {
    let numbers = vec![1,1,1,2,3,4,3,4];
    let result = duplicate_items(numbers);
    let expected_result = vec![1,2,3,4];
    assert_eq!(result, expected_result);
}


#[test]
fn duplicate_items_test_1() {
    let numbers = vec![1,1,1,2,3,4,3,4,10,10,10];
    let result = duplicate_items(numbers);
    let expected_result = vec![1,2,3,4];
    assert_ne!(result, expected_result);
}