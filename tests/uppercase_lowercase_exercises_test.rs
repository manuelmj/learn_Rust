use practical_rust::string_exercises::uppercase_lowercase::{to_uppercase_or_lowercase, UppercaseOrLowercase};


#[test]
fn uppercase_lowercase_test() {
    let result = to_uppercase_or_lowercase("Manuel", UppercaseOrLowercase::Uppercase);
    assert_eq!(result, String::from("MANUEL"));
}

#[test]
fn uppercase_lowercase_test_1() {
    let result = to_uppercase_or_lowercase("Manuel", UppercaseOrLowercase::Lowercase);
    assert_eq!(result, String::from("manuel"));
}

#[test]
fn uppercase_lowercase_test_2() {
    let result = to_uppercase_or_lowercase("Manuel", UppercaseOrLowercase::Uppercase);
    assert_ne!(result, String::from("manuel"));

}