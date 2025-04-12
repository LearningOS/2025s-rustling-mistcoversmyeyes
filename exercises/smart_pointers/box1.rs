// box1.rs
//
/*
 * 本练习演示了 Box<T> 智能指针的重要应用场景：
 *
 * 1. 学习使用 Box<T> 在堆上分配内存
 * 2. 理解如何通过 Box<T> 解决递归类型的大小不确定问题
 * 3. 实现函数式编程中的经典数据结构 cons 列表
 * 4. 掌握创建递归数据结构的模式
 *
 * Box<T> 是 Rust 中最简单的智能指针，主要用于以下场景：
 * - 当编译时无法确定类型大小时（如递归类型）
 * - 当需要在堆上分配大量数据并转移所有权而不复制数据时
 * - 当需要拥有一个实现特定 trait 的类型值但不关心具体类型时
 */
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
