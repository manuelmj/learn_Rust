use practical_rust::string_exercises::password_validation::{validate, PasswordSecurityRequirement};

#[test]
fn string_exercises_test_false() {
    let result = validate(String::from("password"),
     &PasswordSecurityRequirement::new(8,8, 1,1, 1));
    assert_eq!(result, false);
}

#[test]
fn string_exercises_test_true() {
    let result = validate(String::from("Manuel123_ladlk"),
     &PasswordSecurityRequirement::new(8,1, 1,
                                                    1, 1));
    assert_eq!(result, true);   
}