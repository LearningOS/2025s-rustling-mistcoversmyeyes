/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


// 实现插入排序
fn insertion_sort<T>(nums: &mut [T])
// 对array内元素排序，需要元素集合是一个偏序集
where T: std::cmp::PartialOrd 
{
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j - 1] > nums[j] {
            nums.swap(j,j-1);
            j -= 1;
        }
    }
}


fn find_min<T>(nums : &mut [T],start_index : usize)-> usize
where T : std::cmp::PartialOrd
{
    let mut min_index : usize = start_index;
    for i in start_index..nums.len(){
        if nums[min_index] > nums[i]{
            min_index = i;
        }
    }
    min_index
}

// 实现选择排序
fn selection_sort<T>(nums : &mut [T])
where T : std::cmp::PartialOrd
{
    // 初始化排序的区域
    let mut j = 0;
    
    // 从未排序的部分找出最大的元素，与当前已排序的元素的后一个元素交换，扩展已排序的数组的范围
    while j < nums.len(){
        let i = find_min(nums,j);
        nums.swap(j, i);
        j += 1;
    }
}


fn sort<T>(nums:&mut [T])
where T : std::cmp::PartialOrd
{
    selection_sort(nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}