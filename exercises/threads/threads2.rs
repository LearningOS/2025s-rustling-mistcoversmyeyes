/*
 * 本练习主要是要理解如何在多线程环境中安全地更新共享值:
 * 
 * 1. 使用 Arc (Atomic Reference Counting) 在线程间共享数据结构
 * 2. 理解共享可变状态需要某种同步机制 (如 Mutex)
 * 3. 学习如何通过 join 方法等待所有线程完成
 * 4. 观察没有正确同步时可能发生的数据竞争问题
 * 
 * 关键点是在多线程环境中更新共享值时，必须使用适当的同步原语（如 Mutex）
 * 来避免数据竞争，并正确使用 Arc 和 thread::spawn 来创建和管理线程。
 */
// threads2.rs
//
// 基于上一个练习，我们希望所有线程完成它们的工作，但这次生成的线程需要负责更新一个
// 共享值：JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new (JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle  = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
