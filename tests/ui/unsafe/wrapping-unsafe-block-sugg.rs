// run-rustfix

#![deny(unsafe_op_in_unsafe_fn)]

unsafe fn unsf() {}

pub unsafe fn foo() {
    unsf(); //~ ERROR call to unsafe function is unsafe
    unsf(); //~ ERROR call to unsafe function is unsafe
}

fn main() {}
