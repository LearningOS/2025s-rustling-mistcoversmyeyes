// errors3.rs
//
// 这是一个尝试使用上一个练习中完成版的`total_cost`函数的程序。但是它不能正常工作！
// 为什么不行？我们应该怎么做来修复它？
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;

fn main() -> Result <(),ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;//这里脱糖后实际上是使用了 match 匹配 Result 类型处理 Ok 和Err 两个 Variants

    if cost > tokens {
        println!("You can't afford that many!");
        Ok(())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        Ok(())//这个地方真的搞，不同于cpp main 函数返回 0 表示成功执行。rust 返回 Reasult enum 的 Ok(()) 类型标识 正常运行。
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
