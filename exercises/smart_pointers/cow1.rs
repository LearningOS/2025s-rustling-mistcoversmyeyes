// cow1.rs
/*
 * 本练习展示了 Cow (Clone-on-Write) 智能指针的使用：
 *
 * 1. 学习 Cow<T> 的工作原理 - 只在需要修改时才执行克隆操作
 * 2. 理解 Cow 的两种状态：Borrowed (借用) 和 Owned (拥有)
 * 3. 掌握 to_mut() 方法的使用 - 它触发写时的克隆操作
 * 4. 学习如何高效处理可能需要修改的借用数据
 * 5. 理解 Cow 如何帮助优化内存使用和避免不必要的克隆
 *
 * Cow<T> 适用于以下场景：
 * - 处理可能需要修改也可能只需读取的数据
 * - 处理从多种来源（借用或拥有）获取的数据
 * - 需要延迟克隆直到确实需要修改的情况
 * - 需要返回可能是借用也可能是拥有的数据
 * 
 * Cow 是 Rust 标准库中重要的性能优化工具，可以避免不必要的内存分配和复制。
 */
//
// 这个练习探讨了Cow（Clone-On-Write，写时复制）类型。Cow是一种
// 写时复制的智能指针。它可以封装并提供对借用数据的不可变访问，
// 并在需要修改或所有权时懒惰地克隆数据。这种类型设计用于
// 通过Borrow特性处理一般的借用数据。
//
// 这个练习旨在展示将数据传递给Cow时会发生什么。
// 通过在TODO标记处检查Cow::Owned(_)和Cow::Borrowed(_)来修复单元测试。
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err(" "),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated. a
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);   //这里接管了 slice 的所有权
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err(" "),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err(" "),
        }
    }
}
