use std::cell::{Ref, RefCell};
use std::rc::Rc;
fn test_refcell() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();

    println!("{:?}", s1.borrow());    
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

// fn retain_even(nums: &mut Vec<i32>) {
//     let mut i = 0;
//     for num in nums.iter().filter(|&num| is_even(*num)) {
//         nums[i] = *num;
//         i += 1;
//     }
//     nums.truncate(i);
// }

fn retain_even1(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i);
}

use std::cell::Cell;

fn retain_even2(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}


#[derive(Debug)]
struct Info{
    name: RefCell<String>,
    age: i32,
}

fn test_refcell_within_struct(){
    let info = Info{
        name: RefCell::new("123".to_string()),
        age: 88,        
    };

    // modify name 
    *info.name.borrow_mut() =  "345".to_string();
    println!("{:?}",info);

}


#[cfg(test)]
mod tests{

    #[test]
    fn test_refcell() {

        super::test_refcell();
    }

    #[test]
    fn test_refcell_within_struct(){
        super::test_refcell_within_struct()
    }

}