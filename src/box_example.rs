use std::{cell::Cell, sync::{Arc, Mutex}};



// ============= 避免栈上数据的拷贝 ==================
fn test_box_01(){
    let b = Box::new(5);
    println!("b = {}", b);

    let b1  = b;
    println!("b = {}", b1);
}

fn test_box_02() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

// ======== 将动态大小类型变为 Sized 固定大小类型 ==============
/* 
enum List {
    Cons(i32, List), //递归类型 `List` 拥有无限长的大小
    Nil,
}
*/
enum List {
    Cons(i32, Box<List>), //使用 `Box` 将递归类型 `List` 变为固定大小
    Nil,
}

// =========== 特征对象 ============= 

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

fn test_box_03(){
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}), Box::new(Select{id: 2})];
    for e in elems.iter(){
        e.draw();
    }
}


// =========== box 内存布局 =============
fn test_box_04() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
}

// =========== box:leak ============ 
// 在之前的代码中，如果 String 创建于函数中，那么返回它的唯一方法就是转移所有权给调用者 fn move_str() -> String，而通过 Box::leak 我们不仅返回了一个 &str 字符串切片，它还是 'static 生命周期的！
// 使用场景
// 那么我说一个简单的场景，你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak，例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，虽然 Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的。

fn test_box_05() {
    let s = gen_static_str();
    println!("{}", s);
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}


struct Data{
    self_value: u32,
    inner_value: u32,
    d: Option<Box<Data>>,
}

impl Data{
    fn new(self_value: u32,inner_value: u32) -> Data{
        Data{
            self_value,
            inner_value,
            d: None,
        }
    }

    fn set_inner_value(&mut self) {
        if self.d.is_some(){
            let inner_data = self.d.as_mut().unwrap().as_mut();
            inner_data.self_value = self.inner_value;
        }
    }

    fn get_inner_data(&self) -> &Box<Data>{
        self.d.as_ref().unwrap()
    }

}



trait Op{
    fn get_value(&self) -> u32;
}

struct Value{
    value: u32,
}

impl Op for Value{
    fn get_value(&self) -> u32{
        self.value
    }
}   

struct Sub{
    x: Arc<Box<dyn Op>>,
    y: Arc<Box<dyn Op>>,
    res: u32,
}

impl Op for Sub{

    fn get_value(&self) -> u32{
        self.res
    }
}
struct Add{
    x: Arc<Box<dyn Op>>,
    y: Arc<Box<dyn Op>>,
    res: u32,
}

impl Op for Add{
    fn get_value(&self) -> u32{
        self.res
    }
}

struct Mul{
    x: Arc<Box<dyn Op>>,
    y: Arc<Box<dyn Op>>,
    res: u32,
    copy_var: Arc<Mutex<Box<CopyVar>>>,
}   


impl Op for Mul{
    fn get_value(&self) -> u32{
        self.res
    }
}

#[derive(Copy,Clone)]
struct CopyVar{
    value: u32,
}

impl Op for CopyVar{
    fn get_value(&self) -> u32{
        self.value
    }
}

impl Mul{
     fn to_copy(&mut self) -> CopyVar{
        self.copy_var = Box::new(Cell::new(CopyVar{
            value: 0,
        }));

        self.copy_var.as_ref().get()
     }

     fn set_res(&mut self,value: u32){
        self.res = value;
     }

     fn compile(&self) {
        self.copy_var.as_ref().get().value = self.res;
     }  
}
#[cfg(test)]
mod tests{
    use std::{cell::Cell, sync::Arc};

    use super::{Data, Mul};

    #[test]
    fn test_box_mul(){
        let mut v1 = Box::new(super::Value{
            value: 1,
        });

        let mut v2 = Box::new(super::Value{
            value: 2,
        });

        let mut mul = super::Mul{
            x: Arc::new(v1),
            y: Arc::new(v2),
            res: 5,
            copy_var: Box::new(Cell::new(super::CopyVar{
                value: 0,
            })),
        };

        let copy_var = mul.to_copy();

        let add = super::Add{
            x: Arc::new(Box::new(copy_var)),
            y: Arc::new(Box::new(super::Value{
                value: 2,
            })),
            res: 0,
        };

        println!("{:?}",add.x.as_ref().get_value());


        mul.compile();

        println!("{:?}",add.x.as_ref().get_value());
    }

    #[test]
    fn test_box_deep(){
        let mut d = super::Data::new(1,2);
        let mut d1 = super::Data::new(3,4);
        d.d = Some(Box::new(d1));
        d.set_inner_value();
        let data_rep = d.get_inner_data();
        let c= data_rep.to_owned();
    }

    #[test]
    fn test_box_01(){
        super::test_box_01();
    }

    #[test]
    fn test_box_02(){
        super::test_box_02();
    }

    #[test]
    fn test_box_03(){
        super::test_box_03();
    }

    #[test]
    fn test_box_04() {
        super::test_box_04();
    }

    #[test]
    fn test_box_05(){
        super::test_box_05();
    }
}