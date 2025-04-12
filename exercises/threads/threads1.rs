// threads1.rs
/* 这个练习主要是想要你熟悉Rust中线程的基本使用，包括：

1. 使用`thread::spawn`创建新线程，它返回一个`JoinHandle`类型
2. 使用`handle.join()`方法等待线程完成并获取其返回值
3. 理解线程同步的基本概念 - 主线程需要等待所有子线程完成
4. 处理线程可能出现的错误（通过`unwrap()`简单处理）
5. 在多线程环境中安全地收集返回值到一个向量中

这个练习特别强调了如何创建多个线程，让它们并行执行，然后等待它们全部完成并收集它们的结果 - 这是Rust并发编程的基础模式。
 */
// 这个程序生成多个线程，每个线程至少运行250毫秒，并且
// 每个线程返回它们完成所需的时间。该程序应该
// 等待所有生成的线程完成，并将它们的
// 返回值收集到一个向量中。
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles  = vec![];   // 这是存在于进程的堆中的Vec
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now(); // 用于计时的东西
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis() //计算从 start 记录的时间点到当前时间点之间的时间间隔，并将其以毫秒为单位返回。
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        
        // thread::spawn 返回 JoinHandle<Result<T,ERR>> 类型，强制处理时间同步
        // 需要先使用 join 阻塞当前进程等待子线程然后由于子线程有可能会失败所以返回值是 Result<T，E> 强制处理错误。
        results.push(handle.join().unwrap());   
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
