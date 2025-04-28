/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    // 构造一个新的空的Queue
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    /*
    将元素入队
    @ param1 : 
    @ param2 : 
    @ return :() 
     */
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    /* 
    将元素出队
    @ param1: 自己的可变引用
    @ return: 一个Result枚举，用于向外界表明是否出队成功并返回相对应得信息
     */
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    /* 
    @param1: 自己得不可变引用
    @return: Result枚举，如果不为空返回Ok(队首元素),队列为空返回Err(message_str)
     */
    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    /* 
    @param1: 自己Queue的不可变引用
    @return: unsize类型，队中元素个数 */
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /* 
    @param1: 自己的Queue的不可变引用
    @return: bool，如果不为空返回 true，否则false
     */
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/* 
为Queue实现Default Trait，默认创建一个空队列
 */
impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}


pub struct myStack<T>
{
	size : usize,
	q1 : Queue<T>,
	q2 : Queue<T>
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            size :0,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
            self.size += 1;
        }
        else {
            self.q2.enqueue(elem);
            self.size += 1;
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.size == 0 {
            Err("Stack is empty")
        }
        else {
            if !self.q1.is_empty() {
                for count in 1..=self.size -1 {
                    self.q2.enqueue(self.q1.dequeue().unwrap());
                }
                self.size -= 1;
                self.q1.dequeue()
            }
            else {
                for count in 1..=self.size -1 {
                    self.q1.enqueue(self.q2.dequeue().unwrap());
                }
                self.size -= 1;
                self.q2.dequeue()
            }
        }
    }
    pub fn is_empty(&self) -> bool {
		self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}