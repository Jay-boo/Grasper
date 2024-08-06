use tokio::time::error::Elapsed;
use core::fmt;


#[derive(Debug)]
pub enum StreamError<E>{
    TimeoutError(Elapsed),
    SourceError(E)
}


impl<E:fmt::Display>  fmt::Display for StreamError<E>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            StreamError::TimeoutError(err)=>std::fmt::Display::fmt(err, f),
            StreamError::SourceError(e)=>e.fmt(f)
        }
        
    }
}
