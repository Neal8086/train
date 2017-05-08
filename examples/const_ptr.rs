

#[derive(Clone, Debug)]
pub struct S {}

impl std::fmt::Display for S {
     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a S")
    }
}

fn main() {

    let list1: Vec<S> = Vec::new();
    let mut list2: Vec<S> = Vec::new();
    let mut list3: Vec<S> = Vec::new();

    ref_struct(&list1);
    //# ERR: ref_struct(std::ptr::null());

    ref_mut_struct(&mut list2);
    // ERR: ref_mut_struct(std::ptr::null_mut());

   
    ptr_struct(list3.as_mut_ptr() as *const Vec<S>);
    ptr_struct(std::ptr::null());
    //ptr_struct(std::ptr::null_mut());

     ptr_mut_struct(&mut list3);
     // Err: ptr_mut_struct(list3.as_mut_ptr());
     ptr_mut_struct(std::ptr::null_mut());
    
}

fn ref_struct(null_list: &Vec<S>) {

    println!("Null list: {:?}", null_list);
    println!("Null *list: {:?}", *null_list);
}

fn ref_mut_struct(list: &mut Vec<S>) {
    list.push(S {});
}

/// *const 需要是一个 as_mut_ptr, C语言的(可修改的)指针
/// *count 可以传递一个 null 值， 及：std::ptr::null()
fn ptr_struct(list: *const Vec<S>) {

    if list == std::ptr::null() {
        print!("Null - ");
    } 

    println!("Ptr *count: {:?}", list);
}

fn ptr_mut_struct(list: *mut Vec<S>) {
    
    if list == std::ptr::null_mut() {
        print!("Null - ");
    } else {
        unsafe {
            std::ptr::write(list.offset(0), Vec::new());
        }
        print!("Write - ");
    }
    
    println!("Ptr *mut: {:?}", list);
}
