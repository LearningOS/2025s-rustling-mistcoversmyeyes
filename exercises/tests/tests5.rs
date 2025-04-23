// tests5.rs
//
// Rust中的`unsafe`作为一种契约。
//
// 当`unsafe`标记在项目声明上时，如函数、trait等等，它同时声明了一个契约。
// 然而，契约的内容不能仅由一个关键字表达。因此，你有责任在项目的文档注释中
// 的`# Safety`部分手动说明它。
//
// 当`unsafe`标记在由花括号括起的代码块上时，它声明了对某些契约的遵守，
// 比如指针参数的有效性，某个内存地址的所有权等。然而，就像上面的文本一样，
// 你仍然需要在代码块的注释中说明契约是如何被遵守的。
//
// 注意：所有注释都是为了你代码的可读性和可维护性，而Rust编译器将代码健全性的
// 信任交给了你自己！如果你无法证明你自己代码的内存安全和健全性，那就退一步
// 使用安全的代码代替！
//
// 执行`rustlings hint tests5`或使用`hint`观察子命令获取提示。



/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        *(address as *mut u32 ) = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
