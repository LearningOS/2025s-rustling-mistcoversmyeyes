// tests9.rs
//
// Rust具有与C/C++和其他静态编译语言共享FFI接口的强大能力，
// 甚至可以在代码本身内部进行链接！它通过extern块实现，就像下面的代码一样。
//
// extern关键字后的简短字符串表示外部导入函数将遵循的ABI。在本练习中，
// 使用的是"Rust"，而其他变体包括标准C ABI的"C"，Windows ABI的"stdcall"等。
//
// 外部导入的函数在extern块中声明，使用分号而不是大括号标记签名的结束。
// 可以对这些函数声明应用一些属性来修改链接行为，比如#[link_name = ".."]
// 用于修改实际的符号名称。
//
// 如果你想将你的符号导出到链接环境中，也可以在函数定义前标记`extern`关键字，
// 并带有相同的ABI字符串注释。Rust函数的默认ABI实际上是"Rust"，所以如果你
// 想要链接纯Rust函数，整个extern术语可以省略。
//
// Rust默认会对符号进行名称修饰（mangle），就像C++一样。要抑制这种行为并使
// 这些函数可以通过名称寻址，可以应用#[no_mangle]属性。
//
// 在本练习中，你的任务是使测试用例能够调用Foo模块中的`my_demo_function`。
// `my_demo_function_alias`是`my_demo_function`的别名，所以测试用例中的
// 两行代码应该调用同一个函数。
//
// 你不应该修改任何现有代码，只需添加两行属性。

// I AM NOT DONE


extern "Rust" {
    #[link_name = "my_demo_function"] 
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}


mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        // wrap them in safe Rust APIs to ease the burden of callers.
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.w those functions are aliases of a safe
        unsafe {function.
            my_demo_function(123);
            my_demo_function_alias(456);
        }   my_demo_function_alias(456);
    }   
}

