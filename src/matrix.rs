use std::alloc::{alloc, dealloc, Layout};

pub struct Matrix<'a> {
    data: &'a mut u64,
    rows: u32,
    columns: u32,
    total: u32,
}


/*
For the data field we will need a replacement for the _aligned_malloc function available in Windows C++.
There's a crate you can use at https://github.com/jonas-schievink/aligned_alloc.rs
Or you can implement something yourself based on: http://stevenlr.com/posts/handmade-rust-1-allocators/

Rust documentation https://doc.rust-lang.org/nomicon/uninitialized.html
implementation of Rust Vec: https://doc.rust-lang.org/nomicon/vec.html

Eventually, it turned out that the standard lib provides an easy way to allocate aligned memory on the heap...
Double check if this is a suitable analog for the _aligned_alloc() windows function

*/
impl Matrix<'_> {
    pub fn new(rows: u32, columns: u32) -> Self {
        unimplemented!();
    }
}

///
/// Testing the aligned memory allocation provided by the Rust std lib
/// 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_layout() {
        let layout = Layout::from_size_align(1035, 64);
        let layout  = layout.unwrap(); 
        assert!(layout.size() == 1035);
        assert!(layout.align() == 64);
    }
    
    #[test]
    fn aligned_alloc() {
        let layout = Layout::from_size_align(1035, 64);
        let layout  = layout.unwrap(); 
        unsafe {
             let ptr = alloc(layout);
             
             assert!((ptr as usize)  % layout.align() == 0);
             dealloc(ptr, layout);
        }
    }
}