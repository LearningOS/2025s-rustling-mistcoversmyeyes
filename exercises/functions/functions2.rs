// functions2.rs

/**
 * 1.rust的参数列表中必须标明参数类型。
 * 2.rust的for循环不支持面向过程的for循环，而是从面向对象的角度实现了for循环，从“描述遍历的过程”->“描述要便利什么对象”
 * 3.
 */

//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
