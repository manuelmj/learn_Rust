

pub enum UppercaseOrLowercase {
    Uppercase,
    Lowercase
}


pub fn to_uppercase_or_lowercase(string: &str, lowercase: UppercaseOrLowercase) -> String {
    match lowercase {
        UppercaseOrLowercase::Uppercase => string.to_uppercase(),
        UppercaseOrLowercase::Lowercase => string.to_lowercase(),
    }   
}