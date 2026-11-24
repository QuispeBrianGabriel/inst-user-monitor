//! Formato de números y fechas para mostrar.

/// Número con separador de miles: `12345` → `12.345`.
pub fn count(value: i64) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    if value < 0 {
        grouped.push('−');
    }
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push('.');
        }
        grouped.push(digit);
    }
    grouped
}

/// Variación con signo: `+12`, `−3`, `±0`.
pub fn delta(value: i64) -> String {
    match value {
        0 => "±0".to_owned(),
        1.. => format!("+{}", count(value)),
        _ => count(value),
    }
}

/// Tiempo transcurrido legible a partir de los milisegundos.
pub fn elapsed(millis: f64) -> String {
    let minutes = (millis / 60_000.0).floor().max(0.0) as u64;
    let hours = minutes / 60;
    let days = hours / 24;
    match (days, hours, minutes) {
        (_, _, 0) => "hace un momento".to_owned(),
        (_, 0, 1) => "hace 1 minuto".to_owned(),
        (_, 0, m) => format!("hace {m} minutos"),
        (0, 1, _) => "hace 1 hora".to_owned(),
        (0, h, _) => format!("hace {h} horas"),
        (1, _, _) => "ayer".to_owned(),
        (d, _, _) if d < 30 => format!("hace {d} días"),
        (d, _, _) if d < 60 => "hace 1 mes".to_owned(),
        (d, _, _) if d < 365 => format!("hace {} meses", d / 30),
        (d, _, _) if d < 730 => "hace 1 año".to_owned(),
        (d, _, _) => format!("hace {} años", d / 365),
    }
}

/// Fecha y hora locales de una fecha ISO 8601 de la API, p. ej.
/// `24 nov 2026, 18:32`. Fuera del navegador (tests) la devuelve tal cual.
pub fn datetime(iso: &str) -> String {
    if !cfg!(target_arch = "wasm32") {
        return iso.to_owned();
    }
    let date = js_sys::Date::new(&iso.into());
    if date.get_time().is_nan() {
        return iso.to_owned();
    }
    let options = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&options, &"dateStyle".into(), &"medium".into());
    let _ = js_sys::Reflect::set(&options, &"timeStyle".into(), &"short".into());
    date.to_locale_string("es", &options).into()
}

/// Tiempo transcurrido desde una fecha ISO 8601 de la API: `hace 5 minutos`.
/// Fuera del navegador (tests) devuelve un texto vacío.
pub fn since(iso: &str) -> String {
    if !cfg!(target_arch = "wasm32") {
        return String::new();
    }
    let time = js_sys::Date::new(&iso.into()).get_time();
    if time.is_nan() {
        return String::new();
    }
    elapsed(js_sys::Date::now() - time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_with_thousands_separator() {
        assert_eq!(count(0), "0");
        assert_eq!(count(999), "999");
        assert_eq!(count(1000), "1.000");
        assert_eq!(count(1_234_567), "1.234.567");
        assert_eq!(count(-12_345), "−12.345");
    }

    #[test]
    fn deltas_have_a_sign() {
        assert_eq!(delta(0), "±0");
        assert_eq!(delta(12), "+12");
        assert_eq!(delta(-3), "−3");
        assert_eq!(delta(1500), "+1.500");
    }

    #[test]
    fn elapsed_time_reads_naturally() {
        const MINUTE: f64 = 60_000.0;
        const DAY: f64 = 24.0 * 60.0 * MINUTE;
        assert_eq!(elapsed(-5.0), "hace un momento");
        assert_eq!(elapsed(30_000.0), "hace un momento");
        assert_eq!(elapsed(MINUTE), "hace 1 minuto");
        assert_eq!(elapsed(59.0 * MINUTE), "hace 59 minutos");
        assert_eq!(elapsed(60.0 * MINUTE), "hace 1 hora");
        assert_eq!(elapsed(23.0 * 60.0 * MINUTE), "hace 23 horas");
        assert_eq!(elapsed(DAY), "ayer");
        assert_eq!(elapsed(3.0 * DAY), "hace 3 días");
        assert_eq!(elapsed(45.0 * DAY), "hace 1 mes");
        assert_eq!(elapsed(200.0 * DAY), "hace 6 meses");
        assert_eq!(elapsed(400.0 * DAY), "hace 1 año");
        assert_eq!(elapsed(800.0 * DAY), "hace 2 años");
    }
}
