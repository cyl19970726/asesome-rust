#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! example_macro {
    // ident 变量， literal 值
    ($ident:ident, $lit:literal) => { 
        println!("Identifier: {:?}", $ident); 
        println!("Literal: {:?}", $lit);
    };
}
