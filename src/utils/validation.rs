//! Reglas de validación de formularios.
//!
//! Devuelven el mensaje listo para mostrar al usuario. El backend debe repetir
//! estas comprobaciones: la validación en cliente es solo una ayuda de UX.

pub type ValidationResult = Result<(), &'static str>;

/// Política de contraseñas de la API (`models/users/password.rs` del back):
/// si no coincide, el registro pasa la validación aquí y la API lo rechaza.
pub const MIN_PASSWORD_LEN: usize = 12;
pub const MAX_PASSWORD_LEN: usize = 128;

pub fn validate_email(email: &str) -> ValidationResult {
    let email = email.trim();
    if email.is_empty() {
        return Err("Introduce tu correo electrónico.");
    }
    if is_valid_email(email) {
        Ok(())
    } else {
        Err("El correo electrónico no es válido.")
    }
}

/// Comprobación estructural razonable (`local@dominio.tld`), no RFC 5322 completa.
fn is_valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    let labels: Vec<&str> = domain.split('.').collect();

    !local.is_empty()
        && !email.chars().any(char::is_whitespace)
        && !domain.contains('@')
        && labels.len() >= 2
        && labels.iter().all(|label| !label.is_empty())
}

/// Para iniciar sesión solo se exige que no esté vacía.
pub fn validate_password_present(password: &str) -> ValidationResult {
    if password.is_empty() {
        Err("Introduce tu contraseña.")
    } else {
        Ok(())
    }
}

/// Requisitos de una contraseña nueva: los mismos que la API y en el mismo
/// orden, para que el primer fallo que se muestra sea el que daría ella.
pub fn validate_new_password(password: &str) -> ValidationResult {
    if password.is_empty() {
        Err("Introduce una contraseña.")
    } else if password.len() > MAX_PASSWORD_LEN {
        Err("No puede superar 128 caracteres.")
    } else if !password.chars().all(|c| c == ' ' || c.is_ascii_graphic()) {
        Err("Solo admite letras sin tilde, números, símbolos y espacios.")
    } else if password.len() < MIN_PASSWORD_LEN {
        Err("Debe tener al menos 12 caracteres.")
    } else if !password.chars().any(|c| c.is_ascii_lowercase()) {
        Err("Debe incluir al menos una minúscula.")
    } else if !password.chars().any(|c| c.is_ascii_uppercase()) {
        Err("Debe incluir al menos una mayúscula.")
    } else if !password.chars().any(|c| c.is_ascii_digit()) {
        Err("Debe incluir al menos un número.")
    } else if !password
        .chars()
        .any(|c| c.is_ascii_punctuation() || c == ' ')
    {
        Err("Debe incluir al menos un símbolo.")
    } else {
        Ok(())
    }
}

pub fn validate_password_confirmation(password: &str, confirmation: &str) -> ValidationResult {
    if confirmation.is_empty() {
        Err("Repite la contraseña.")
    } else if password != confirmation {
        Err("Las contraseñas no coinciden.")
    } else {
        Ok(())
    }
}

/// Fortaleza estimada de una contraseña, para el medidor del registro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PasswordStrength {
    Empty,
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    pub fn evaluate(password: &str) -> Self {
        if password.is_empty() {
            return Self::Empty;
        }

        let len = password.chars().count();
        let classes = [
            password.chars().any(|c| c.is_lowercase()),
            password.chars().any(|c| c.is_uppercase()),
            password.chars().any(|c| c.is_ascii_digit()),
            password.chars().any(|c| !c.is_alphanumeric()),
        ]
        .into_iter()
        .filter(|&present| present)
        .count();

        let score = classes + usize::from(len >= MIN_PASSWORD_LEN) + usize::from(len >= 16);
        match score {
            0..=2 => Self::Weak,
            3 => Self::Fair,
            4 => Self::Good,
            _ => Self::Strong,
        }
    }

    /// Nivel de 0 a 4 (segmentos encendidos en el medidor).
    pub fn level(self) -> u8 {
        self as u8
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Empty => "",
            Self::Weak => "Débil",
            Self::Fair => "Aceptable",
            Self::Good => "Buena",
            Self::Strong => "Fuerte",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_emails() {
        for email in [
            "ana@example.com",
            "a.b+tag@sub.domain.io",
            "  ana@example.com  ",
        ] {
            assert!(validate_email(email).is_ok(), "{email}");
        }
    }

    #[test]
    fn rejects_invalid_emails() {
        for email in [
            "",
            "ana",
            "ana@",
            "@example.com",
            "ana@example",
            "ana@@example.com",
            "a na@example.com",
            "ana@example..com",
        ] {
            assert!(validate_email(email).is_err(), "{email}");
        }
    }

    #[test]
    fn new_password_rules() {
        assert!(validate_new_password("Abcdefgh123!").is_ok());
        assert!(
            validate_new_password("Abcdefgh 123").is_ok(),
            "el espacio cuenta como símbolo"
        );
        for (password, why) in [
            ("", "vacía"),
            ("Abcdef12!", "corta"),
            ("Ábcdefgh123!", "con tilde"),
            ("abcdefgh123!", "sin mayúscula"),
            ("ABCDEFGH123!", "sin minúscula"),
            ("Abcdefghijk!", "sin número"),
            ("Abcdefgh1234", "sin símbolo"),
        ] {
            assert!(
                validate_new_password(password).is_err(),
                "{why}: {password}"
            );
        }
        assert!(validate_new_password(&format!("Aa1!{}", "x".repeat(125))).is_err());
        assert!(validate_new_password(&format!("Aa1!{}", "x".repeat(124))).is_ok());
    }

    #[test]
    fn password_confirmation() {
        assert!(validate_password_confirmation("abc12345", "abc12345").is_ok());
        assert!(validate_password_confirmation("abc12345", "").is_err());
        assert!(validate_password_confirmation("abc12345", "abc12346").is_err());
    }

    #[test]
    fn password_strength_grows_with_complexity() {
        assert_eq!(PasswordStrength::evaluate(""), PasswordStrength::Empty);
        assert_eq!(PasswordStrength::evaluate("abc"), PasswordStrength::Weak);
        assert_eq!(
            PasswordStrength::evaluate("abcd1234"),
            PasswordStrength::Weak
        );
        assert_eq!(
            PasswordStrength::evaluate("Abcd1234"),
            PasswordStrength::Fair
        );
        assert_eq!(
            PasswordStrength::evaluate("Abcd1234!"),
            PasswordStrength::Good
        );
        assert_eq!(
            PasswordStrength::evaluate("Abcd1234!xyz"),
            PasswordStrength::Strong
        );
        assert_eq!(PasswordStrength::Strong.level(), 4);
    }
}
