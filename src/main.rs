pub mod lifetime;
pub mod marco_test;
pub mod closure_example;    
pub mod type_sys;
pub mod try_from_to;
pub mod box_example;
pub mod deref_example;
pub mod rc_example;
pub mod cell_example;
pub mod unsafe_example;
pub mod dyn_trait_example;
// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         // 绘制按钮的代码
//     }
// }

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         // 绘制SelectBox的代码
//     }
// }

// // pub struct Screen {
// //     pub components: Vec<Box<dyn Draw>>,
// // }

// // impl Screen {
// //     pub fn run(&self) {
// //         for component in self.components.iter() {
// //             component.draw();
// //         }
// //     }
// // }

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No")
//                 ],
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }   


pub mod trait_defaultfunction_example{
    pub trait Summary {
        // 默认实现
        fn summarize(&self) -> String{
            String::from("(Reading)")
        }
    }
    pub struct Post {
        pub title: String, // 标题
        pub author: String, // 作者
        pub content: String, // 内容
    }
    
    // impl Summary for Post {} 假如是这么实现的话会使用默认实现
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }
    
    pub struct Weibo {
        pub username: String,
        pub content: String
    }
    
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    
    
}

pub mod dyn_trait_example_01{
    trait Draw {
        fn draw(&self) -> String;
    }
    
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    
    // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    fn draw1(x: Box<dyn Draw>) {
        // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
        x.draw();
    }
    
    fn draw2(x: &dyn Draw) {
        x.draw();
    }
    
    pub fn run_dyn_trait_example_01() {
        let x = 1.1f64;
        // do_something(&x);
        let y = 8u8;
    
        // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw> 
        // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
        draw1(Box::new(x));
        // 基于 y 的值创建一个 Box<u8> 类型的智能指针
        draw1(Box::new(y));
        draw2(&x);
        draw2(&y);
    }
}
    

pub mod dyn_trait_example_02{
    pub trait Draw {
        fn draw(&self) -> String;
    }
    
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,//这个定义是 静态Trait 
    // }

//     impl<T> Screen<T>
//         where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>, //这个定义是动态Trait, 会有 ptr和 vptr
    }
    
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    
    pub fn run_dyn_trait_example_02(){
        let screen = Screen{
            components: vec![
                Box::new(
                  8u8
                ),
                Box::new(
                    1.1f64
                ),
            ]
        };
    }
    
    
}
use dyn_trait_example_01::*;
use dyn_trait_example_02::*;
fn main(){
    let vec_test = vec2!(1,2,3);
    vec_test.iter().for_each(|value|{println!("{:?}",value);});
    run_dyn_trait_example_01();
    run_dyn_trait_example_02();
}
//  https://pic1.zhimg.com/80/v2-b771fe4cfc6ebd63d9aff42840eb8e67_1440w.jpg