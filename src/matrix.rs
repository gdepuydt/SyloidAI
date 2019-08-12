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
*/
impl Matrix<'_> {
    pub fn new(rows: u32, columns: u32) -> Self {
        unimplemented!();
    }
}