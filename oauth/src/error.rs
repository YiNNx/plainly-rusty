#[derive(Debug)]
pub struct ErrorCustom(pub String);

impl std::fmt::Display for ErrorCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ErrorCustom {}
