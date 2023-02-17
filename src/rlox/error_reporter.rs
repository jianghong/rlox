pub struct ErrorReporter {
    pub had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter { had_error: false }
    }

    pub fn error(&mut self, line: u32, message: &String) {
        self.report(line, &"".to_string(), message);
    }

    pub fn report(&mut self, line: u32, place: &String, message: &String) {
        println!("[line {line}] Error {place}: {message}");
        self.had_error = true;
    }
}