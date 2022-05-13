pub use self::BinaryTree::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}
