// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

/* 
rust内置类型:array
1. 语法格式：[Type;Number]
2. 构造方式：
    - 显式列出所有的元素 如：[1,2,3]
    - 使用重复语法 如：[0,1000] 标识数组里有1000个'0'
    - 使用宏或者库（这里不像python 那样可以轻松写一个循环在数组里面构造，但却比cpp灵活，给了一步到位的方法）*/
fn main() {
    let a = [0; 1000];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
