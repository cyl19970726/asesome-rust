use std::borrow::Borrow;

struct Data{
    left: u32,
    right: u32,
}

// impl Borrow<Data> for [u32]{
//     fn borrow(&self) -> &Data{
//         assert_eq!(self.len(), 2);
//         unsafe {
//             &*(self.as_ptr() as *const Data)
//         }
//     }
// }

impl Borrow<Data> for [u32]{
    fn borrow(&self) -> &Data{
        let (prefix, shorts, suffix) = unsafe { self.align_to::<Data>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}


impl Data {
    fn new(left: u32, right: u32) -> Data{
        Data{left, right}
    }

    fn print(&self) {
        println!("left: {}, right: {}", self.left, self.right);
    }
}

#[cfg(test)]
mod tests{
    use crate::unsafe_example::Data;
    use super::*;
    use std::borrow::*;

    #[test]
    fn test_align_to(){
        unsafe {
            let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
            let (prefix, shorts, suffix) = bytes.align_to::<u16>();
            println!("{:?}", prefix);
            println!("{:?}", shorts);
            println!("{:?}", suffix);
            // less_efficient_algorithm_for_bytes(prefix);
            // more_efficient_algorithm_for_aligned_shorts(shorts);
            // less_efficient_algorithm_for_bytes(suffix);
        }
    }

    #[test]
    fn test_borrow(){
        let arr = [1u32,2u32].as_slice();
        let data:&Data = (*arr).borrow();
        data.print();
    }
}
