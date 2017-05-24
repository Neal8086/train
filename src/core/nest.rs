use NsError;
use NsResult;
use Config;


pub struct Nest {}

impl Nest {
     pub fn new(config: &Config) -> Nest {
         Nest {}
     }

     pub fn filter(&self) -> &Nest {
         self
     }

     pub fn listen(&self) -> NsResult<i32> {
         Ok(0)
     }
}