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

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            // En caso de que el árbol no esté vacío lo introduccimos según esperado
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

pub fn create_tree_with_add<'a>() -> BinaryTree<&'a str> {
    let mut tree: BinaryTree<&'a str> = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    tree.add("Mars");
    tree.add("Uranus");
    tree.add("Saturn");
    tree.add("Jupyter");
    tree
}
