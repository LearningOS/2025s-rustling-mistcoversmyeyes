// box1.rs
//
// 在编译时，Rust 需要知道一个类型占用多少空间。这对于递归类型来说
// 会变得有问题，因为一个值可以包含同类型的另一个值作为其一部分。为了
// 解决这个问题，我们可以使用 `Box` - 一种用于在堆上存储数据的智能指针，
// 它也允许我们包装递归类型。
//
// 在这个练习中我们要实现的递归类型是 `cons 列表` - 一种在函数式编程语言中
// 经常见到的数据结构。cons 列表中的每个项目包含两个元素：当前项目的值和
// 下一个项目。最后一项是一个叫做 `Nil` 的值。
//
// 步骤 1：在枚举定义中使用 `Box` 使代码能够编译
// 步骤 2：通过替换 `todo!()` 来创建空的和非空的 cons 列表
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    return List::Nil
}

pub fn create_non_empty_list() -> List {
    let result =  List::Cons(1,Box::new(List::Nil));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
