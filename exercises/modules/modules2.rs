/// # 模块与导入
///
/// 本练习展示了如何使用Rust的模块系统：
/// 
/// ## 关键概念
/// 
/// 1. `mod` - 定义一个模块
/// 2. `use` - 将模块路径引入作用域
/// 3. `as` - 为导入的项目提供别名
///
/// ## 解决方案说明
///
/// 需要解决的问题是：
/// - 将`delicious_snacks::fruits::PEAR`引入作用域并重命名
/// - 将`delicious_snacks::veggies::CUCUMBER`引入作用域并重命名
/// - 这些重命名后的常量需要能被`main`函数中的`delicious_snacks::fruit`和`delicious_snacks::veggie`访问
///
// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
