
mod ios {
    pub const aa: i32 = 1;
}

mod ss {
    pub const bb: i32 = 0;
}

use ios::*;
use ss::*;


fn main() {
    println!("Two modules: {:?}, {:?}", aa, bb);
}