/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    /* 
    @param1: 接受两个不可变模板元素的闭包，闭包应当返回bool类型，用于比较两个模板元素顺序。
    @return: 全新的空Heap
     */
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    /* 
    @param1: 自身的不可变引用
    @return: unsize。堆的大小
     */
    pub fn len(&self) -> usize {
        self.count
    }

    /* 
    @param1: 自身的不可变引用
    @return: bool。堆是否为空
     */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /* 
    @param1: 自身的可变引用
    @param2: 模板元素T。要添加的元素。
    @return: ()。
     */
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut new_item_idx = self.count;
        let mut cur_item_par_indx = self.parent_idx(new_item_idx);
        while !((self.comparator)(&self.items[cur_item_par_indx],&self.items[new_item_idx])) && new_item_idx > 1{
            self.items.swap(cur_item_par_indx,new_item_idx);
            new_item_idx = cur_item_par_indx;
            cur_item_par_indx = self.parent_idx(new_item_idx);
        }
    }

    /* 
    @param1: 自身的不可变引用。
    @param2: unsize。当前节点下标。
    @return: unsize。当前节点的父节点的下标。
     */
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /* 
    @param1: 自身的不可变引用
    @param2: unsize。当前节点下标。
    @return: unsize。当前节点的左子节点的下标。
     */
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }
    
    
    /* 
    @param1: 自身的不可变引用
    @param2: unsize。当前节点下标。
    @return: unsize。当前节点的右子节点的下标。
     */
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /* 
    TOREVIEW
    @param1:
    @param2: unsize。当前节点下标。
    @return: unsize。 以当前节点为根的堆的最小元素的下标。
    */
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // 如果右子节点不存在，返回左子节点
        if right_idx > self.count {
            return left_idx;
        }
        
        // 根据比较器选择左子节点或右子节点
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

// TOREVIEW
impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 取出第一个有效元素（索引1是堆的根节点）
        let mut result = T::default();
        std::mem::swap(&mut result, &mut self.items[1]);
        
        // 将最后一个元素移到根节点
        if self.count > 1 {
            self.items[1] = self.items.pop().unwrap();
        } else {
            self.items.pop();
            self.count -= 1;
            return Some(result);
        }
        
        self.count -= 1;
        
        // 从根节点开始向下调整堆
        let mut current_idx = 1;
        
        // 只要当前节点有子节点，就继续调整
        while self.children_present(current_idx) {
            let smallest_child_idx = self.smallest_child_idx(current_idx);
            
            // 如果当前节点需要和子节点交换
            if !(self.comparator)(&self.items[current_idx], &self.items[smallest_child_idx]) {
                self.items.swap(current_idx, smallest_child_idx);
                current_idx = smallest_child_idx;
            } else {
                break; // 堆性质已满足，退出循环
            }
        }
        
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}