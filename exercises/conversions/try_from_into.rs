// try_from_into.rs
//
/*
 * 本练习演示了 Box<T> 智能指针的重要应用场景：
 *
 * 1. 学习使用 Box<T> 在堆上分配内存
 * 2. 理解如何通过 Box<T> 解决递归类型的大小不确定问题
 * 3. 实现函数式编程中的经典数据结构 cons 列表
 * 4. 掌握创建递归数据结构的模式
 *
 * Box<T> 是 Rust 中最简单的智能指针，主要用于以下场景：
 * - 当编译时无法确定类型大小时（如递归类型）
 * - 当需要在堆上分配大量数据并转移所有权而不复制数据时
 * - 当需要拥有一个实现特定 trait 的类型值但不关心具体类型时
 */
// 
// TryFrom 是一种简单且安全的类型转换，在某些情况下可能会以受控的方式失败。基本上，这与 From 相同。
// 主要区别在于这应该返回 Result 类型，而不是目标类型本身。你可以在以下链接阅读更多信息：
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}


// 你的任务是完成此实现并返回一个内部类型为 Color 的 Ok 结果。你需要为三个整数的元组、t
// 三个整数的数组和整数切片创建实现。
//
// 注意，元组和数组的实现将在编译时进行检查，但切片实现需要检查切片长度！另外请注意，
// 正确的 RGB 颜色值必须是 0..=255 范围内的整数。


// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // 分别检查3个数字的范围
        if tuple.0 <= 255 && tuple.1 <= 255 &&tuple.2 <= 255
        && tuple.0 >= 0 && tuple.1 >=0 && tuple.2 >= 0{
                return Result::Ok(Color{red : tuple.0.try_into().unwrap(),green : tuple.1.try_into().unwrap(),blue : tuple.2.try_into().unwrap()})
        }
        else {
            return Err(IntoColorError::IntConversion);
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // 检查三个数据的范围支不支持准确转化到u8
        if arr.iter().all(|&x| x >= 0 && x <= 255){
            Result::Ok(
                Color{  
                        red : arr[0].try_into().unwrap(),
                        green : arr[1].try_into().unwrap(),
                        blue : arr[2].try_into().unwrap()
                    }
            )
        }
        else {
            Err(Self::Error::IntConversion)
        }
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            Err(IntoColorError::BadLen)
        }
        else if slice.iter().any(|&x| x < 0 || x >255) {
            Err(IntoColorError::IntConversion)
        }
        else {
            Ok(
                Color{
                    red : slice[0].try_into().unwrap(),
                    green : slice[1].try_into().unwrap(),
                    blue : slice[2].try_into().unwrap()
                }
            )
        }
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
