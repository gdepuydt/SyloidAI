use std::alloc::{alloc, dealloc, Layout};
use std::ops::Index;

#[derive(PartialEq)]
pub struct Matrix {
    data: *mut f32,
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
        let data_layout = Layout::from_size_align(std::mem::size_of::<f32>() * total as usize, 64).unwrap();
        Matrix {
            rows,
            columns,
            total: total,
            data_layout: data_layout,
            data: alloc(data_layout) as *mut f32,
        }
    }

    pub fn get(&self, row: usize, column: usize) -> *mut f32 {
        if self.rows <= row || self.columns <= column {
            panic!("Requested element ouside Matrix index bounds.");
        }

        let index = (&self.columns * row) + column;
        unsafe {
            self.data.add(index) as *mut f32
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
        for r in 1..a.rows {
            for c in 1..b.columns {
                let mut total: f32 = 0.0;
                for d in 1..shared_dimension {
                    //We need a way to access the memory in data with an index (that will be calculated using row and column count)
                    //Unsure how to do this in Rust at the moment.
                    //I assume this here is how we have to do it: https://doc.rust-lang.org/std/primitive.pointer.html#method.add
                    //More specifically we could probably use: https://doc.rust-lang.org/std/ops/trait.Index.html
                    unsafe {
                        total += *(a.get(r, d)) * *(b.get(d, c));
                        *(self.get(r,c)) = total;
                    }                   
                }
            }
        }
    }

    pub fn add(&self, a: &Matrix) {
        if self.rows != a.rows || self.columns != a.columns {
            panic!("Matrices must be of the same dimensions to perform matrix addition.");
        }

        for i in 0..self.total-1 {
            unsafe {
                *(self.data.add(i)) += *(a.data.add(i))
            }
        }
    }

    pub fn sub(&self, a: &Matrix) {
        if self.rows != a.rows || self.columns != a.columns {
            panic!("Matrices must be of the same dimensions to perform matrix subtraction.");
        }

        for i in 0..self.total-1 {
            unsafe {
                *(self.data.add(i)) -= *(a.data.add(i))
            }
        }
    }

    pub fn hadamard(&self, a: &Matrix) {
        if self.rows != a.rows || self.columns != a.columns {
            panic!("Matrices must be of the same dimensions to perform Hadamard.");
        }
        
        for i in 0..self.total-1 {
            unsafe {
                *(self.data.add(i)) *= *(a.data.add(i))
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

// Overwriting the index [] operator is an option, but matrices are '2D' with rows and columns, so, a 1D index lacks clarity...Why not just use methods?
// Or have a google search for what is common fr working with matrices in Rust...

impl Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &*(self.data.add(index))
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