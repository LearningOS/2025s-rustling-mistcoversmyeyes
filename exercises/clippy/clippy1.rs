// clippy1.rs
//
// Clippy 工具是一个 lint 集合，用于分析您的代码，以便您可以
// 捕获常见错误并改进您的 Rust 代码。
// 
// 对于这些练习，当有 clippy 警告时，代码将无法编译
// 请查看输出中 clippy 的建议来解决练习。
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
