// quiz2.rs
//
// 这是针对以下章节的小测验：
// - 字符串
// - 向量
// - 所有权移动
// - 模块
// - 枚举
//
// 让我们以函数的形式构建一个小机器。作为输入，我们将
// 提供一个字符串和命令的列表。这些命令决定对字符串
// 应用什么操作。可以是：
// - 将字符串转为大写
// - 修剪字符串的空白
// - 将"bar"附加到字符串后指定的次数
// 具体形式将是：
// - 输入将是一个二元组的向量，
//   第一个元素是字符串，第二个是命令。
// - 输出元素将是一个字符串的向量。
//
// 这次没有提示！

// 我还没完成

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 待办：完成函数签名！
    pub fn transformer(input: Vec<(String,Command)> ) -> Vec<String> {
        // 待办：完成输出声明！
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // 待办：完成函数体。你能做到的！
            match command {
                Command::Uppercase =>{
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(num) =>{
                    let bars = "bar".repeat(*num);
                    output.push(string.to_string() + &bars);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 待办：我们需要导入什么才能让`transformer`在作用域内？
    use super::Command;
    // use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let output = crate::my_module::transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
