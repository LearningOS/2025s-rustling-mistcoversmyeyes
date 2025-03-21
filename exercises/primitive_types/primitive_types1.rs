// primitive_types1.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types1` or use the `hint` watch subcommand
// for a hint.


/* 
这一个章节主要讲的是rust的内置数据类型。
我们都知道cpp中有几大primitive-types：
1.整数类型
    - char：int8
    - short: int16 
    - int: int32
    - long: int 64
    - long long :int 128
2. 浮点数类型：
    - float
    - double
3. 布尔类型：
    - bool
4. 空类型
    - void
5. 空指针类型 (C++11):
    - std::nullptr_t
  */


fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = true;// Finish the rest of this line like the example! Or make it be false!
    if !is_evening {
        println!("Good evening!");
    }
}
