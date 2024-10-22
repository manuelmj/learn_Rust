use std::collections::HashMap;
use practical_rust::string_exercises::count_words::count_words;


#[test]
fn count_words_test() {
    let paragraph = String::from("Manuel");
    let result:HashMap<&str, u32> = count_words(paragraph.as_str());

    assert_eq!(result, HashMap::from([("Manuel", 1)]));
}

#[test]
fn count_words_test_1() {
    let paragraph = String::from("Manuel Manuel Manjarres Rivera el el mejor mejor mejor");
    let result:HashMap<&str, u32> = count_words(paragraph.as_str());

    assert_eq!(result, HashMap::from([("Manuel", 2), ("Manjarres", 1), ("Rivera", 1), ("el", 2), ("mejor", 3)]));
}   

#[test]
fn count_words_test_2() {
    let paragraph = String::from("Manuel Manuel Manjarres Rivera el el mejor mejor mejor");
    let result:HashMap<&str, u32> = count_words(paragraph.as_str());

    assert_ne!(result, HashMap::from([("Manuel", 2), ("Manjarres", 1), ("Rivera", 1), ("el", 1), ("mejor", 2)]));
}   