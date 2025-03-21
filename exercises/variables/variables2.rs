// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x = 10;     //使用 let 关键字声明一个常量对象，不强制指定类型，由编译器自动推断类型。实际上展开为 let inmut x :i32 = 10;。
    if x == 10 {    //rust 语言的分支结构中的条件判断语句不用括号括起来
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
