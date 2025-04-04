// errors6.rs
//
// 对于库代码，不推荐使用像 `Box<dyn error::Error>` 这样的全捕获错误类型，
// 因为调用者可能希望根据错误内容做出决策，而不是简单地打印或进一步传播它。
// 在这里，我们定义了一个自定义错误类型，使调用者能够在我们的函数返回错误时
// 决定下一步该做什么。
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
    fn from_parseint (err : ParseIntError ) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    // 上边的语法糖展开后效果如下：
    // let x : i64 = match s.parse {
    //     Ok(v) => v,
    //     Err(e) => return e,
    // };
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
