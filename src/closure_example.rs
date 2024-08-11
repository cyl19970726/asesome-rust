struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn example_01(){
    let mut cacher = Cacher::new(|x| x);
    let v1 = cacher.value(1);
    let v2 = cacher.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}


// 捕获作用域中的值
fn example_02(){
    let x = 4;
    let euqal_to_x = |z| z == 4;
    let y = 4;
    assert!(euqal_to_x(y));
}

// 闭包对内存的影响
// 当闭包从环境中捕获一个值时，会分配内存去存储这些值。对于有些场景来说，这种额外的内存分配会成为一种负担。与之相比，函数就不会去捕获这些环境值，因此定义和使用函数不会拥有这种内存负担。


fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    // 仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用
    // println!("{}", func(4));
}

pub fn example_03() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}

// ==== fnMut ==== 
// ，闭包自动实现Copy特征的规则是，只要闭包捕获的类型都实现了Copy特征的话
fn fnmut_example() {
    let mut s = String::new();
    // 因此这里捕获了它的可变借用
    let update_string =  |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}

mod fnmove{
    fn move_test() {
        // 拿所有权
        let s = String::new();
        let update_string = move || println!("{}", s);
        // let update_string = || println!("{}", s);

        exec1(update_string);
        // exec2(update_string); // 不能再用了 因为前面使用了move 

        // 可变引用
        let mut s = String::new();
        let mut update_string = || s.push_str("hello");
        exec2(update_string);
        // exec1(update_string); // 不能再用了
    }
    
    // fn exec<'a, F: Fn(&'a str)>(mut f: F)  {
    //     f("hello")
    // }

    fn exec1<F: Fn()>(f: F) {
        f()
    }

    fn exec2<F: FnMut()>(mut f:F){
        f()
    }

    #[cfg(test)]
    mod tests{

        use super::*;

        #[test]
        fn test_for_move_test(){
            move_test()
        }
    }
}
// ==== fnMut and fn ==== 
// ，闭包自动实现Copy特征的规则是，只要闭包捕获的类型都实现了Copy特征的话
fn fnmut_fn_example() {
    let mut s = String::new();
    let s1 = "word".to_string();
    // 因此这里捕获了它的可变借用
    let update_string =  |str1| {
        s.push_str(str1);
        println!("{:?}",s1);  //s 是可变借用，但是 s1不是不是，但是这个闭包仍然需要是FnMut
    };

    exec1(update_string);

    println!("{:?}",s);
}

fn exec1<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}

// 三种 Fn 的关系
// 实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：

// 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
// 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
// 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征

fn diff_in_fn_trait() {
    let s = String::new();

    let update_strings = || println!("{:?}",s);

    diff_exec(update_strings);
    diff_exec1(update_strings);
    diff_exec2(update_strings);
    
}

fn diff_exec<F: FnOnce()>(f:F) {
    f()
}
fn diff_exec1<F: FnMut()>(mut f: F)  {
    f()
}
fn diff_exec2<F: Fn()>(f: F)  {
    f()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_cacher_value() {
        let mut cacher = Cacher::new(|x| x);
        let v1 = cacher.value(1);
        let v2 = cacher.value(2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_equal_to_x() {
        let x = 4;
        let equal_to_x = |z| z == 4;
        let y = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn test_fn_once() {
        // 仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用
        example_03();
    }

    #[test]
    fn test_fn_mut(){
        fnmut_example();
    }

    #[test]
    fn test_fn_fnmut_example(){
        fnmut_fn_example();
    }

    #[test]
    fn test_diff_fn_trait(){
        diff_in_fn_trait();
    }
}


