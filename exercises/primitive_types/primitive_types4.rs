// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

/* 语法糖："num1...num2"表示num1到num2 */
#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; //和cpp的迭代器逻辑一样，涉及复制就是左闭右开，参考电脑光标的选中逻辑。

    assert_eq!([2, 3, 4], nice_slice)
}
