use NsResult;
use Config;


pub struct Nest {
    config: Config,
}

impl Nest {
     pub fn new(config: &Config) -> Nest {
        trace!("Nest init");

        Nest {
            config: config.clone(),
        }
     }

     pub fn module(&self) -> &Nest {
        trace!("Nest module");
        trace!("Config addr: {:?}", self.config.addr);

         self
     }

     pub fn listen(&self) -> NsResult<i32> {
         trace!("Nest listen");
         
         Ok(0)
     }

    
}