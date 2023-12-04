#[derive(Debug)]
pub struct NotImplementedErrorType;

impl std::fmt::Display for NotImplementedErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The Solution is not implemented yet")
    }
}

impl std::error::Error for NotImplementedErrorType {}
