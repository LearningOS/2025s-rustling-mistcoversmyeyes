// using_as.rs
/*
 * 本练习演示了 Rust 中 `as` 运算符的使用：
 *
 * 1. 学习使用 `as` 进行基本的类型转换（从一种数值类型到另一种）
 * 2. 理解整数除法和浮点数除法的区别
 * 3. 掌握如何处理不同数值类型间的运算（特别是整数和浮点数混合计算）
 * 4. 学习避免因类型不匹配导致的编译错误
 * 5. 理解 Rust 不会自动进行某些潜在有损失的类型转换
 *
 * `as` 运算符适用于简单的、显式的类型转换场景：
 * - 在数值类型之间转换（如 i32 到 f64 或 usize 到 i32）
 * - 扩大或缩小数值范围
 * - 在需要特定类型参数的上下文中进行必要的类型调整
 * 
 * 使用 `as` 时需要注意可能的精度损失或溢出问题，对于复杂类型，
 * 应该使用更安全的 From/Into 或 TryFrom/TryInto 特性。
 */
//
// Rust中的类型转换是通过使用`as`运算符完成的。请注意，
// `as`运算符不仅用于类型转换。它还有助于重命名导入。
//
// 目标是确保除法不会编译失败，
// 并返回正确的类型。
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.


fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total /values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
