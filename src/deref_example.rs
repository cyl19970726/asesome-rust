// 何为智能指针？能不让你写出 ****s 形式的解引用，我认为就是智能: )，智能指针的名称来源，主要就在于它实现了 Deref 和 Drop 特征，这两个特征可以智能地帮助我们节省使用上的负担：

// Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
// Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

use std::ops::Deref;

fn test_deref_01() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // 这里 y 就是一个常规引用，包含了值 5 所在的内存地址，然后通过解引用 *y，我们获取到了值 5。如果你试图执行 assert_eq!(5, y);，代码就会无情报错，因为你无法将一个引用与一个数值做比较
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t:T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn test_deref_02() {
    let y = MyBox::new(5);

    assert_eq!(5, *y);
}

// ===================== 函数和方法中的隐式 Deref 转换 ==================
/* 
* 背后的原理
当我们对智能指针 Box 进行解引用时，实际上 Rust 为我们调用了以下方法：

*(y.deref())
首先调用 deref 方法返回值的常规引用，然后通过 * 对常规引用进行解引用，最终获取到目标值。
*/

fn test_deref_03() {
    let s = String::from("hello world");
    display(&s)
}
// 以上代码有几点值得注意：

// String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
// &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
// 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)
/** 
 #[stable(feature = "rust1", since = "1.0.0")]
impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
} 
*/
fn display(s: &str) {
    println!("{}",s);
}


// 连续的隐式 Deref 转换



#[cfg(test)]
mod tests{
    #[test]
    fn test_deref_01() {
        super::test_deref_01();
    }

    #[test]
    fn test_deref_02() {
        super::test_deref_02();
    }

    #[test]
    fn test_deref_03() {
        super::test_deref_03();
    }
}