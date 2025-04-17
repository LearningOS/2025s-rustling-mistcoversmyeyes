// from_str.rs
//
/*
 * FromStr 特性实现练习
 * 
 * 这个练习旨在教授以下 Rust 知识点：
 *
 * 1. FromStr trait 与 parse() 方法的孪生关系：
 *    - FromStr 是一个 trait，定义如何从字符串创建类型
 *    - 一旦实现了 FromStr，类型就自动获得了通过 str.parse::<Type>() 方法创建的能力
 *    - 这两者内在相连：parse() 方法内部调用 FromStr::from_str 实现
 * 
 * 2. 自定义错误类型设计：
 *    - 使用枚举表达不同类型的解析错误
 *    - 为每种错误情况提供具体错误类型而非通用错误
 *    - 包装和传递底层错误（如数字解析错误）
 *
 * 3. 错误处理模式：
 *    - 与 From/Into 不同，这里返回 Result 而非默认值
 *    - 使用 Result<T, E> 类型显式传播错误
 *    - 通过详细错误类型提供更好的错误信息
 *
 * 4. 字符串解析的常见模式：
 *    - 分割字符串并验证元素数量
 *    - 检查空值和无效输入
 *    - 解析转换特定数据类型（如字符串到数字）
 *
 * 5. 强类型系统的应用：
 *    - 使用明确的类型转换代替隐式转换
 *    - 利用类型系统确保解析的安全性
 *    - 通过编译时检查避免运行时错误
 */
// 这与 from_into.rs 类似，但这次我们将实现 `FromStr` 并返回错误，
// 而不是回退到默认值。此外，通过实现 FromStr，你可以在字符串上使用 `parse` 方法
// 来生成实现类型的对象。你可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html
// 上阅读更多信息
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// 步骤：
// 1. 如果提供的字符串长度为0，应该返回一个错误
// 2. 根据字符串中的逗号分割字符串
// 3. 分割后应该只返回2个元素，否则返回一个错误
// 4. 从分割操作中提取第一个元素作为姓名
// 5. 从分割操作中提取另一个元素，并将其解析为`usize`类型的年龄，
//    类似于`"4".parse::<usize>()`
// 6. 如果在提取姓名和年龄的过程中出现问题，应该返回一个错误
// 如果一切顺利，则返回一个Person对象的Result
//
// 顺便说一下：`Box<dyn Error>`实现了`From<&'_ str>`。这意味着如果
// 你想返回一个字符串错误消息，你可以通过
// return `Err("我的错误消息".into())`来实现。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0{
            return Err(Self::Err::Empty);
        }
        else  {
            let parts : Vec<&str> = s.split(',').collect();
            if parts.len() != 2{
                return Err(ParsePersonError::BadLen);
            }
            else {
                let name = parts[0];
                if name.is_empty(){
                    return Err(Self::Err::NoName);
                }

                match parts[1].parse::<usize>() {
                    Ok(age)=>Ok(Person{
                            name : name.to_string(),
                            age : age
                        }),
                    Err(e) =>{
                        Err(ParsePersonError::ParseInt(e))
                    }
                }
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
