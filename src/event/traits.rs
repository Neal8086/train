use NsError;
use NsResult;

pub trait NsEventTrait: Sized {

    fn new() -> NsResult<Self>;

    fn add_event();

    fn del_event();

    fn notify();
    
    fn process_events();
}
