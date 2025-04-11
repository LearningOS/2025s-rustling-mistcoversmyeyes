// strings2.rs
//
// 修改代码使其编译通过，不要改变函数签名！
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    // let attempt : &String = &word;      // rust中的引用需要被显式地创建，而不像cpp那样可以作为一个临时值传入函数中
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
