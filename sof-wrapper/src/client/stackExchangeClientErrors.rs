use std::fmt::{ Debug, Display};

use tokio::time::error::Elapsed;


pub enum StreamError{
    TimeoutError(Elapsed),
    FetchError(FetchError)
    
}

pub struct FetchError{
    msg: String
}

impl FetchError {
    pub fn new(msg:&str)->Self{
        FetchError { msg: msg.to_string() }
    }
    
}

impl Debug for FetchError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"FetchError : {}",self.msg)
    }
}
impl From<reqwest::Error> for FetchError{
    fn from(value: reqwest::Error) -> Self {
        FetchError { msg: value.to_string()}
    }
}
impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::TimeoutError(e) => write!(f, "Timeout Error: {}", e),
            StreamError::FetchError(e) => write!(f, "Fetch Error: {}", e.msg),
        }
    }
}

