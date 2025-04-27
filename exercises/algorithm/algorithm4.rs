/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


// 引入Ordering 模块 和 Debug 模块
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            },
            Some(ref mut root) => {
                root.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&mut self, value: T) -> bool {
        match self.root{
            None => {
                false
            }
            Some(ref mut cur_node_ptr) => {
                cur_node_ptr.as_mut().search(value)
            }
            
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree,Assume that the tree rooted at curNode NonEmpty。
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value){
            Ordering::Less => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some (ref mut left_node) => {
                        left_node.as_mut().insert(value);
                    }
                }
            },
            Ordering::Equal => {
                
            },
            Ordering::Greater => {
                match self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut right_node) =>{
                        right_node.as_mut().insert(value);
                    }
                }
            }
        }
    }
    
    fn search(&mut self,value:T) -> bool{
        match self.value.cmp(&value){
            Ordering::Equal => {
                true
            }
            Ordering::Less => {
                match self.right {
                    None => {
                        false
                    }
                    Some(ref mut right_node_ptr) => {
                        right_node_ptr.as_mut().search(value)
                    }
                }
                
            }
            Ordering::Greater => {
                match self.left {
                    None => {
                        false
                    }
                    Some(ref mut left_node_ptr) => {
                        left_node_ptr.as_mut().search(value)
                    }
                }
            }
        }


        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


