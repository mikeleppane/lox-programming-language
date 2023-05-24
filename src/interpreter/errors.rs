pub fn report(line: usize, place: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, place, message);
}
