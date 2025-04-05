// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
//     software.licensing_info() == software_two.licensing_info()
// }

// 第二种修改方法, 但是要同步地修改函数调用时候的写法。这种方法包装为第一种方法的语法糖。
fn compare_license_types<T : Licensed,U: Licensed>(software: T, software_two: U) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

/*  编译报错
error[E0782]: expected a type, found a trait
  --> exercises/traits/traits4.rs:26:36
   |
26 | fn compare_license_types(software: Licensed, software_two: Licensed) -> bool {
   |                                    ^^^^^^^^
   |
help: you can also use an opaque type, but users won't be able to specify the type parameter when calling the `fn`, having to rely exclusively on type inference
   |
26 | fn compare_license_types(software: impl Licensed, software_two: Licensed) -> bool {
   |                                    ++++
help: alternatively, use a trait object to accept any type that implements `Licensed`, accessing its methods at runtime using dynamic dispatch
   |
26 | fn compare_license_types(software: &dyn Licensed, software_two: Licensed) -> bool {
   |                                    ++++
 */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
