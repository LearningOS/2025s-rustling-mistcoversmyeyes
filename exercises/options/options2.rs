// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {//这一步实际上可以看作是尝试赋值的操作。如果“let Some（word） = optional_target”  成功。那么说明 optional_target 是 Option::Some 类型的
            assert_eq!(word, target)
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 不要被这个Some(Some(interger)) 绕进去了。已知调用 Vec<T> 的 pop 方法会返回 Some(T) ,然后这里的T 是Some(i8) 于是返回Some(Some(i8))
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
