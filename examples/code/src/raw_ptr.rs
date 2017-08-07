use std::collections::LinkedList;

#[derive(Debug)]
struct A {
    pub f: i32,
}

fn main() {

    let mut a = A { f:1 };
    let a_raw: *mut A = &mut a;

    let mut b = A { f:2 };
    let b_raw: *mut A = &mut b;

    let mut list1 = LinkedList::new();
    list1.push_back(a_raw);
    list1.push_back(b_raw);

    println!("current: {:?}", list1);
    println!("front: {:?}", list1.front());
    println!("back: {:?}", list1.back());

    let new_a_raw = (*list1.front().unwrap()) as *mut A;

    unsafe {
        //let ref_mut: &mut A = &mut *a_raw;
        let ref_mut: &mut A = &mut *new_a_raw;
        
        println!("{:?}, {:?}, {:?}", new_a_raw, a_raw, ref_mut);
    }
}