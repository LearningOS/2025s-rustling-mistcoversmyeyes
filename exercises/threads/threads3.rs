// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;        // 导入多生产者单消费者通道，用于线程间通信
use std::sync::Arc;         // 导入原子引用计数，允许多个线程安全地共享所有权
use std::thread;            // 导入线程相关功能
use std::time::Duration;    // 导入时间持续相关类型，用于线程休眠

// 定义一个队列结构体，包含长度和两个部分的数据
struct Queue {
    length: u32,            // 队列的总长度
    first_half: Vec<u32>,   // 队列的前半部分
    second_half: Vec<u32>,  // 队列的后半部分
}

impl Queue {
    // 实现创建新队列的方法
    fn new() -> Self {
        Queue {
            length: 10,                     // 设置队列长度为10
            first_half: vec![1, 2, 3, 4, 5], // 前半部分包含1到5
            second_half: vec![6, 7, 8, 9, 10], // 后半部分包含6到10
        }
    }
}

// 函数接收一个队列和一个发送者，创建线程发送队列中的数据
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);       // 将队列包装在Arc中以实现线程间共享
    let qc1 = Arc::clone(&qc);  // 为第一个线程克隆Arc引用
    let qc2 = Arc::clone(&qc);  // 为第二个线程克隆Arc引用

    let tx1 = tx.clone();
    // 创建第一个线程处理队列的前半部分
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);  // 打印正在发送的值
            tx.send(*val).unwrap();         // 通过通道发送值
            thread::sleep(Duration::from_secs(1)); // 线程休眠1秒
        }
    });

    // 创建第二个线程处理队列的后半部分
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);  // 打印正在发送的值
            tx1.send(*val).unwrap();         // 通过通道发送值
            thread::sleep(Duration::from_secs(1)); // 线程休眠1秒
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel(); // 创建通道
    let queue = Queue::new();      // 创建新队列
    let queue_length = queue.length; // 获取队列长度

    send_tx(queue, tx); // 调用函数发送队列数据

    let mut total_received: u32 = 0; // 初始化接收计数
    for received in rx {             // 遍历接收到的数据
        println!("Got: {}", received); // 打印接收到的值
        total_received += 1;          // 增加接收计数
    }

    println!("total numbers received: {}", total_received); // 打印总接收数
    assert_eq!(total_received, queue_length) // 断言接收数等于队列长度
}
