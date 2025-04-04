// errors2.rs
//
// 假设我们正在开发一个游戏，你可以用代币购买物品。所有物品都需要5个代币，
// 而且每次购买物品时都有1个代币的处理费。游戏玩家会输入他们想要购买的物品数量，
// 然后`total_cost`函数将计算代币的总成本。由于玩家输入的是数量，所以我们得到的是字符串
// —— 他们可能输入任何内容，不仅仅是数字！
//
// 目前，这个函数完全没有处理错误情况（而且也没有正确处理成功的情况）。我们想要做的是：
// 如果我们对一个不是数字的字符串调用`parse`函数，该函数将返回一个`ParseIntError`，
// 在这种情况下，我们希望立即从我们的函数返回该错误，而不尝试进行乘法和加法运算。
//
// 至少有两种方法可以实现这一点，两种都是正确的 —— 但其中一种要简短得多！
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;


/* 
    @parameter : 
    @return : 
 */
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;         // 处理费用
    let cost_per_item = 5;          // 每一件物品所要消耗的费用
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
