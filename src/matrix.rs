use std::alloc::{alloc, dealloc, Layout};

pub struct Matrix {
    data: *mut u8,
    data_layout: Layout,
    rows: usize,
    columns: usize,
    total: usize,
}


/*
For the data field we will need a replacement for the _aligned_malloc function available in Windows C++.
There's a crate you can use at https://github.com/jonas-schievink/aligned_alloc.rs
Or you can implement something yourself based on: http://stevenlr.com/posts/handmade-rust-1-allocators/

Rust documentation https://doc.rust-lang.org/nomicon/uninitialized.html
implementation of Rust Vec: https://doc.rust-lang.org/nomicon/vec.html

Eventually, it turned out that the standard lib provides an easy way to allocate aligned memory on the heap...
Double check if this is a suitable analog for the _aligned_alloc() windows function

Furthermore, it turns out the call to alloc requires unsafe Rust, which is to be avoided as much as possible.

*/
impl Matrix {
    pub unsafe fn new(rows: usize, columns: usize) -> Self {
        let total = rows * columns;
        let data_layout = Layout::from_size_align(std::mem::size_of::<u64>() * total, 64).unwrap();
        Matrix {
            rows,
            columns,
            total: total,
            data_layout: data_layout,
            data: alloc(data_layout),
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data, self.data_layout);
        }
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