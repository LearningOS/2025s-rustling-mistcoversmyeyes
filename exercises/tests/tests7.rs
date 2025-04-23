// tests7.rs
//
// 在构建包时，有些依赖既不能在`Cargo.toml`中导入，也不能直接链接；
// 一些预处理过程从代码生成到设置包特定的配置都有所不同。
//
// Cargo并不旨在替代其他构建工具，但它通过自定义构建脚本`build.rs`与它们集成。
// 这个文件通常放在项目的根目录，而在这个案例中，它与本练习位于同一目录。
//
// 它可以用于：
//
// - 构建捆绑的C库。
// - 在主机系统上查找C库。
// - 从规格说明生成Rust模块。
// - 执行该crate需要的任何平台特定配置。
//
// 当设置配置时，我们可以在构建脚本中使用`println!`来告诉Cargo遵循某些指令。
// 通用格式是：
//
//     println!("cargo:{}", your_command_in_string);
//
// 有关构建脚本的更多信息，请参阅官方Cargo书籍：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在这个练习中，我们寻找一个环境变量并期望它在一个范围内。
// 你可以查看测试用例来了解详细信息。
//
// 你不应该修改这个文件。修改同一目录下的`build.rs`文件来通过这个练习。
//
// 执行`rustlings hint tests7`或使用`hint` watch子命令获取提示
// hint.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
