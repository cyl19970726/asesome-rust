#![feature(const_trait_impl)]

use std::{cell::Cell as Cell_std, mem};

pub trait Borrow<Borrowed: ?Sized>{
    fn borrow(&self) -> &Borrowed;
}

pub trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed>{
    fn borrow_mut(&mut self) -> &mut Borrowed;
}

impl<T: ?Sized> Borrow<T> for T{
    fn borrow(&self) -> &T{
        self
    }
}

impl<T: ?Sized> BorrowMut<T> for T{
    fn borrow_mut(&mut self) -> &mut T{
        self
    }
}

//每一个类型的引用都实现了对自身的Borrow trait
impl<T: ?Sized> Borrow<T> for &T {
    fn borrow(&self) -> &T {
        &**self
    }
}
//每一个类型的可变引用都实现了针对自身的Borrow trait
impl<T: ?Sized> Borrow<T> for &mut T {
    fn borrow(&self) -> &T {
        &**self
    }
}

//每一个类型的可变引用都实现了针对自身的BorrowMut
impl<T: ?Sized> BorrowMut<T> for &mut T {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
impl<T> UnsafeCell<T> {
    //创建封装结构
    pub const fn new(value: T) -> UnsafeCell<T> {
        UnsafeCell { value }
    }

    //解封装
    pub fn into_inner(self) -> T {
        self.value
    }
}
//对任意T的类型，可以为T.into() 创建UnsafeCell类型变量
impl<T> From<T> for UnsafeCell<T> {
    fn from(t: T) -> UnsafeCell<T> {
        UnsafeCell::new(t)
    }
}

impl<T: ?Sized> UnsafeCell<T> {

    pub fn from_mut(value: &mut T) -> &mut UnsafeCell<T> {
       unsafe { &mut *(value as *mut T as *mut UnsafeCell<T>) }
    }
    pub const fn get(&self) -> *mut T {
        // 将裸指针导出，这是为什么起名是UnsafeCell的原因
        // 此裸指针的安全性由调用代码保证,调用代码可以使用此裸指针改变内部封装的变量
        self as *const UnsafeCell<T> as *const T as *mut T
    }

    //给出一个正常的可变引用, 此引用存在期间，get及raw_get调用会编译器告警
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    //参数与get有区别，是关联函数
    pub const fn raw_get(this: *const Self) -> *mut T {
        this as *const T as *mut T
    }
}

//显然，UnsafeCell不支持Sync，即使内部变量支持Sync，这与RUST的默认规则不一致，需要显式声明
// impl <T:?Sized> !Sync for UnsafeCell<T> {} 

pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}


impl<T> From<T> for Cell<T> {
    fn from(t: T) -> Cell<T> {
        Cell::new(t)
    }
}

impl<T> Cell<T> {
    pub const fn new(value: T) -> Cell<T> {
        Cell { value: UnsafeCell::new(value) }
    }

    pub fn set(&self, val: T) {
        self.replace(val);
    }

    pub fn replace(&self, val: T) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        mem::replace(unsafe { &mut *self.value.get() }, val)
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        unsafe { *self.value.get() }
    }

    pub fn update<F>(&self, f: F) -> T
    where
        F: FnOnce(T) -> T,
    {
        let old = self.get();
        let new = f(old);
        self.set(new);
        new
    }
}


impl<T: ?Sized> Cell<T> { 

    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }


    pub fn from_mut(t: &mut T) -> &Cell<T> {
        // SAFETY: `&mut` ensures unique access.
        unsafe { &*(t as *mut T as *const Cell<T>) }
    }
}

mod tests{

    use super::*;

    #[test]
    fn test_cell(){
        let cell = Cell::new(1);
        assert_eq!(cell.get(), 1);
        cell.set(2);
        assert_eq!(cell.get(), 2);
        cell.update(|x| x + 1);
        assert_eq!(cell.get(), 3);
    }

    #[test]
    fn test_unsafe_cell(){
        let cell = UnsafeCell::new(1);
        assert_eq!(unsafe { *cell.get() }, 1);
        unsafe { *cell.get() = 2 };
        assert_eq!(unsafe { *cell.get() }, 2);
    }

    #[test]
    fn test_unsafe_cell_mut(){
        let mut value = 1;
        let cell = UnsafeCell::from_mut(&mut value);
        assert_eq!(unsafe { *cell.get() }, 1);
        unsafe { *cell.get() = 2 };
        assert_eq!(unsafe { *cell.get() }, 2);
    }

    #[test]
    fn test_cell_set(){
        let c1 = Cell::new(2);
        c1.set(3);
        assert_eq!(c1.get(), 3);
    }

    #[test]
    fn test_multi_mut_ref(){
        let c2 = Cell::new(2);

        let mut_ref= c2.as_ptr();
        let mut_ref2 = c2.as_ptr();

        unsafe { *mut_ref = 3 };
        unsafe { *mut_ref2 = 4 };
        assert_eq!(c2.get(), 4);
    }

    #[test]
    fn test_for_multi_mut2(){
        let mut a = 3u32;
        let a_ref= &mut a;

        let mut_ref = unsafe {
            &mut *(a_ref as *const u32 as *mut u32)
        };

        let mut_ref2 = unsafe {
            &mut *(a_ref as *const u32 as *mut u32)
        };

        *mut_ref = 4;
        assert_eq!(a,4);

        *mut_ref2 = 5;
        assert_eq!(a,5);

        // *a_ref = 6;
        // assert_eq!(a,6);
    }

}