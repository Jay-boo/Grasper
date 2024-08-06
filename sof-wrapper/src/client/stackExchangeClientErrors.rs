use std::fmt::{ Debug, Display};


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

