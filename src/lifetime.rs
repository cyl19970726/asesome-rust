#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        // 将可变借用变为不可变借用
        &*self
    }
    fn share(&self) {}
}

// fn life_time_test_1() {
//     let mut foo = Foo;
//     let loan = foo.mutate_and_share();
//     foo.share();
//     println!("{:?}", loan);
// }
