// errors5.rs
//
// 该程序使用了errors4中代码的修改版本。
//
// 本练习使用了一些我们在课程后面才会学到的概念，比如`Box`和`From`特性。现在详细理解它们并不重要，
// 但如果你愿意的话可以提前阅读。目前，可以将`Box<dyn ???>`类型视为"我想要任何实现了???"的类型，
// 考虑到Rust通常的运行时安全标准，这应该算是相当宽松的！
//
// 简而言之，这种特殊的box用例是为了当你想要拥有一个值，并且你只关心它是一个实现了特定特性的类型。
// 为此，Box被声明为Box<dyn Trait>类型，其中Trait是编译器在该上下文中使用的任何值上寻找的特性。
// 对于本练习，该上下文是可以在Result中返回的潜在错误。
//
// 我们可以用什么来描述这两种错误？换句话说，有没有一个特性是这两种错误都实现了的？
//
// 执行`rustlings hint errors5`或使用`hint`子命令获取提示。

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
// Box<dyn error::Error> 类型是一个箩筐，可以实现多态，不管返回 的 Err(e) 的 e 是啥类型，都不会报类型匹配的错误。
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
