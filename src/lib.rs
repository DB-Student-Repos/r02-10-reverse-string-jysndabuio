pub fn reverse(input: &str) -> String {
    let mut container = String::new();
    container.extend(input.chars().rev());
    container
}
