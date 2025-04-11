// traits1.rs
//
// 是时候实现一些特性了！你的任务是为类型 `String` 实现特性 `AppendBar`。
// 特性(trait) AppendBar 只有一个函数，它将 "Bar" 附加到任何实现此特性的对象上。
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.



trait AppendBar {
    fn append_bar(self) -> Self; // 函数声明
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) ->Self{
        self.push_str("Bar");
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
