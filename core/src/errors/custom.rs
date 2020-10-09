use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    msg: String,
    side: CustomErrorSide,
}

impl CustomError {
    pub fn new(msg: String) -> Box<CustomError> {
        Box::new(CustomError {
            msg,
            side: CustomErrorSide,
        })
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
struct CustomErrorSide;

impl fmt::Display for CustomErrorSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Side message goes here!")
    }
}

impl Error for CustomErrorSide {}
