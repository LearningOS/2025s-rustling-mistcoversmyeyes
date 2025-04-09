// arc1.rs
//
// 在本练习中，我们有一个名为"numbers"的u32类型的Vec，其值范围从0到99 -- [0, 1, 2, ..., 98, 99]
// 我们希望在8个不同的线程中同时使用这组数字。每个线程将获取每八个值的总和，并带有一个偏移量。
//
// 第一个线程（偏移量0）将求和 0, 8, 16, ...
// 第二个线程（偏移量1）将求和 1, 9, 17, ...
// 第三个线程（偏移量2）将求和 2, 10, 18, ...
// ...
// 第八个线程（偏移量7）将求和 7, 15, 23, ...
//
// 由于我们使用线程，我们的值需要是线程安全的。因此，我们使用Arc。
// 我们需要在两个TODO处进行修改。
//
// 通过在第一个TODO注释处为`shared_numbers`填入一个值，并在第二个TODO注释处为`child_numbers`
// 创建一个初始绑定，使这段代码能够编译。尽量不要创建`numbers` Vec的任何副本！
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

// Arc类型的创建函数
// 直接包装已有值
// Arc::new(T)
// Arc::from(&str)
// 从其他指针转换
// Arc::from(OtherPointer)
// 克隆已有指针
// let arc1 = arc2.clone()


fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
