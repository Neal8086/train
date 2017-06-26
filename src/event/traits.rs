use NsError;
use NsResult;

pub trait NsEventTrait: Sized {

    fn new() -> NsResult<Self>;

    fn add_event(&self);

    fn del_event(&self);

    fn notify_init(&self) -> NsResult<i32>;

    fn notify(&self);
    
    fn process_events(&self);
}
