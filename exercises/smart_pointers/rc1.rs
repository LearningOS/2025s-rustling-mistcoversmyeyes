// rc1.rs
/*
 * 本练习通过太阳系模型演示了 Rc<T> 智能指针的使用场景：
 * 
 * 1. 学习使用 Rc<T> 实现多所有权 - 一个值可以被多个变量同时拥有
 * 2. 理解 Rc 的引用计数机制 - 调用 Rc::clone 增加引用计数
 * 3. 观察 drop 对引用计数的影响 - 当一个所有者被丢弃时，引用计数减少
 * 4. 掌握 Rc::strong_count 检查引用计数的方法
 * 
 * Rc<T> 适用于需要在程序的多个部分共享只读数据的场景，如配置信息、
 * 共享缓存等，但不允许可变引用。它是单线程环境下共享所有权的首选方法。
 */
// 在这个练习中，我们想通过 Rc<T> 类型来表达多个所有者的概念。这是一个太阳系的模型 - 有一个 Sun 类型和
// 多个 Planet。这些行星拥有太阳的所有权，表明它们围绕太阳运转。
//
// 通过使用适当的 Rc 原语来表达太阳有多个所有者，使这段代码能够编译。
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.


// Rc的全称是Reference count,是带有引用计数的智能指针

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    // TODO
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    // TODO
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    // TODO
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    // TODO
    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    // TODO
    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    // TODO
    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
