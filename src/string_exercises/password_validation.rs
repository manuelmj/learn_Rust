


pub struct PasswordSecurityRequirement {
    pub length: i32,
    pub min_uppercase: i32,
    pub min_lowercase: i32,
    pub min_digit: i32,
    pub min_special: i32,
}

impl PasswordSecurityRequirement {
    pub fn new(length: i32, min_uppercase: i32, min_lowercase: i32, min_digit: i32, min_special: i32) -> PasswordSecurityRequirement {
        PasswordSecurityRequirement {
            length,
            min_uppercase,
            min_lowercase,
            min_digit,
            min_special,
        }
    }
}

struct  PasswordRequirement{
    // ... agregar mas validaciones segun la condicion contadores para cantidad de datos deseados
    count_uppercase: i32,
    count_lowercase: i32,
    count_digit: i32,
    count_special: i32,
}

impl PasswordRequirement {
    fn new_all_cero() -> PasswordRequirement {
        PasswordRequirement {
            count_uppercase: 0,
            count_lowercase: 0,
            count_digit: 0,
            count_special: 0,
        }
    }

    fn validate_security(&self, security: &PasswordSecurityRequirement) -> bool {
        self.count_lowercase >= security.min_lowercase && 
        self.count_uppercase >= security.min_uppercase && 
        self.count_special >= security.min_special && 
        self.count_digit >= security.min_digit 
    }

    fn count_lowercase(&mut self) {self.count_lowercase += 1;}
    fn count_uppercase(&mut self) {self.count_uppercase += 1;}
    fn count_special(&mut self) {self.count_special += 1;}
    fn count_digit(&mut self) {self.count_digit += 1;}
}




pub fn validate(password: String, security: &PasswordSecurityRequirement) -> bool {
    // debe tener al menos 8 caracteres, contener al menos dos mayusculas,
    // dos minusculas, dos digitos y  1 caracter especiales
    let mut has_requirement:PasswordRequirement = PasswordRequirement::new_all_cero();

    if password.len() < security.length as usize { return false; }
    
    
    for c in password.chars() {
        if c.is_ascii_uppercase() {
            has_requirement.count_uppercase();
        } else if c.is_ascii_lowercase() {
            has_requirement.count_lowercase();
        } else if c.is_ascii_digit() {
            has_requirement.count_digit();
        } else {
            has_requirement.count_special();
        }
    }

    has_requirement.validate_security(security)
    
}