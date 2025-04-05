// traits3.rs
//
// 你的任务是为两个结构体实现Licensed特性，并让它们返回相同的信息，而不需要编写相同的函数两次。
//
// 考虑你可以向Licensed特性中添加什么。
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String{
        "Some information".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
