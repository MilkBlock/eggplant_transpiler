fn normalize_identifier(identifier: &str) -> String {
    identifier
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn main() {
    println!("?a -> {}", normalize_identifier("?a"));
    println!("?b -> {}", normalize_identifier("?b"));
    println!("? -> {}", normalize_identifier("?"));
    println!("is_alphanumeric('?'): {}", '?'.is_alphanumeric());
}
