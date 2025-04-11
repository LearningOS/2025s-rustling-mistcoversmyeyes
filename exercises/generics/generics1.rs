// generics1.rs
//
// 这个购物清单程序无法编译！使用你对泛型的了解来修复它。
//
// 执行 `rustlings hint generics1` 或使用 `hint` watch 子命令获取提示。


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
