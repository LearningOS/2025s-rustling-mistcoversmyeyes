// tests3.rs
//
// 这个测试没有测试我们的函数 -- 使它以一种使测试通过的方式测试函数。
// 然后编写第二个测试，测试当我们调用 `is_even(5)` 时是否得到我们期望的结果。
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.



pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(4));
    }
}
