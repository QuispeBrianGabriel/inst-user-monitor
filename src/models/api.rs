use std::fmt;

/// Errores de las peticiones autenticadas a la API que la UI sabe mostrar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    /// El token caducó o ya no es válido: hay que volver a iniciar sesión.
    Unauthorized,
    /// El recurso no existe (o es de otro usuario).
    NotFound,
    /// El cuerpo supera el límite de la API.
    TooLarge,
    /// Rechazo de la API: su código estable y su mensaje legible.
    Rejected { code: String, message: String },
    /// La API respondió con un error interno (5xx) o un cuerpo inesperado.
    Server(u16),
    /// La API no responde (red, DNS, CORS...).
    Network,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized => f.write_str("Tu sesión ha caducado. Vuelve a iniciar sesión."),
            Self::NotFound => f.write_str("La cuenta no existe o ya se eliminó."),
            Self::TooLarge => f.write_str("El texto es demasiado grande para enviarlo."),
            Self::Rejected { message, .. } => f.write_str(&sentence(message)),
            Self::Server(status) => {
                write!(f, "Error del servidor ({status}). Inténtalo más tarde.")
            }
            Self::Network => f.write_str("No se pudo conectar con el servidor."),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<gloo_net::Error> for ApiError {
    fn from(_: gloo_net::Error) -> Self {
        Self::Network
    }
}

/// Los mensajes de la API van en minúscula y sin punto final: los muestra
/// como una frase.
fn sentence(message: &str) -> String {
    let message = message.trim();
    let mut chars = message.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut text: String = first.to_uppercase().chain(chars).collect();
    if !text.ends_with(['.', '!', '?']) {
        text.push('.');
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_messages_read_as_sentences() {
        let error = |message: &str| ApiError::Rejected {
            code: "invalid_format".to_owned(),
            message: message.to_owned(),
        };
        assert_eq!(
            error("la línea 1 («Seguidores») no es válida").to_string(),
            "La línea 1 («Seguidores») no es válida."
        );
        assert_eq!(error("ya está.").to_string(), "Ya está.");
        assert_eq!(error("").to_string(), "");
    }
}
