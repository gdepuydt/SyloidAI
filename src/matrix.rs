use std::alloc::{alloc, dealloc, Layout};

#[derive(PartialEq)]
pub struct Matrix {
    data: *mut f32,
    data_layout: Layout,
    rows: usize,
    columns: usize,
    total: f32,
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
        let total = rows as f32 * columns as f32;
        let data_layout = Layout::from_size_align(std::mem::size_of::<f32>() * total as usize, 64).unwrap();
        Matrix {
            rows,
            columns,
            total: total,
            data_layout: data_layout,
            data: alloc(data_layout) as *mut f32,
        }
    }

    pub fn mul(&self, a: &Matrix, b: &Matrix) {
            // pre-requisite that columns of a match number of rows in b: What is the Rust idiom for implementing this, including error handling...
            // For now, being novice, let's just use the panic!() function.AsMut
            // For reference, this blogpost gives an excellent overview of idiomatic error handling in Rust:AsMut
            // https://blog.burntsushi.net/rust-error-handling/

            if a.columns != b.rows {
                panic!("Number of columns Matrix A must match number of columns Matrix B.");
            }

            if self == a || self == b {
                panic!("Destination matrix must be distinct from matrices A and B.");
            }
            
            let shared_dimension = a.rows;
            for _ in 1..a.rows {
                for _ in 1..b.columns {
                    let mut total: f32 = 0.0;
                    for _ in 1..shared_dimension {
                        //We need a way to access the memory in data with an index (that will be calculated using row and column count)
                        //Unsure how to do this in Rust at the moment.
                        //I assume this here is hoz we have to do it: https://doc.rust-lang.org/std/primitive.pointer.html#method.add
                        //!!TODO!!
                    }
                }
            }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data as *mut u8, self.data_layout);
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