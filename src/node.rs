use std::cmp;
use std::mem;

type OptBoxNode<K,D> = Option<Box<Node<K,D>>>;

#[derive(Default)]
pub struct Node<K, D> {
    pub key: K,
    pub data: D,

    pub height: isize,

    pub left: OptBoxNode<K,D>,
    pub right: OptBoxNode<K,D>,
}

use std::fmt;
impl<K,D> fmt::Debug for Node<K,D> 
where K: fmt::Debug, D: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = match &self.left {
            Some(node) => format!("Node {{ {:?}:{:?} }}", node.key, node.data),
            None => String::from("None"),
        };
        let right = match &self.right {
            Some(node) => format!("Node {{ {:?}:{:?} }}", node.key, node.data),
            None => String::from("None"),
        };
        write!(f, "{{ {:?}:{:?}, left: {:?}, right: {:?} }}", &self.key, &self.data, left, right)
    }
}

impl<K,D> Node<K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    pub fn new(key: K, data: D) -> Self {
        Self { key, data, height: 1, left: None, right: None }
    }

    fn height(&mut self) -> isize {
        // cache result from potentially expensive drill-down
        // TODO: when does this need to be invalidated?
        if self.height != 0 {
            return self.height;
        }

        self.update_height()
    }

    fn update_height(&mut self) -> isize {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(node) = &mut self.left {
            left_height = node.height() + 1;
        } 
        if let Some(node) = &mut self.right {
            right_height = node.height() + 1;
        } 

        self.height = cmp::max(left_height, right_height);
        self.height
    }

    /// return the difference in height between the right tree and the left tree
    /// a positive value indicates that the right tree is deeper
    /// a negative value indicates that the left tree is deeper
    pub fn balance_factor(&mut self) -> isize {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(node) = &mut self.left {
            left_height = node.height();
        }
        if let Some(node) = &mut self.right {
            right_height = node.height();
        }

        right_height - left_height
    }

    /// recursively search for the given key
    pub fn find(&self, key: K) -> Option<&D> {
        if key == self.key {
            return Some(&self.data);
        } else if key < self.key {
            if let Some(node) = &self.left {
                return node.find(key);
            } else { 
                return None; 
            }
        } else { // key > self.key
            if let Some(node) = &self.right {
                return node.find(key);
            } else { 
                return None; 
            }
        }
    }

    /// insert a new key/data pair
    pub fn put(&mut self, key: K, data: D) -> Result<(), String> {
        self.height = 0;

        if key == self.key {
            self.data = data;
        } else if key < self.key {
            // key is less than self.key
            if let Some(node) = &mut self.left {
                // if we have a left node, recurse to it
                node.put(key, data)?;
                self.height = node.height() + 1;
            } else {
                // otherwise, create it
                self.left = Some(Box::new(Node::new(key, data,)));
            }
        } else {
            // key is greater than self.key
            if let Some(node) = &mut self.right {
                // if we have a right node, recurse to it
                node.put(key, data)?;
                self.height = node.height() + 1;
            } else {
                // otherwise, create it
                self.right = Some(Box::new(Node::new(key, data,)));
            }
        }

        //self.rebalance();

        Ok(())
    }

    pub fn left_heavy(&mut self) -> bool {
        if self.balance_factor() < -1 { return true; }
        else { return false; }
    }
    pub fn right_heavy(&mut self) -> bool {
        if self.balance_factor() > 1 { return true; }
        else { return false; }
    }


    /* right rotation after a node is inserted in the left subtree of a left subtree
     * left rotation after a node is inserted in the right subtree of a right subtree
     * left-right rotation after a node is inserted as the right subtree of a left subtree
     * right-left rotation after a node is inserted as the left subtree of a right subtree
     */


}


/*             root                  left
 *            /                     /    \
 *           left    =>     left_left    root
 *          /
 * left_left
 *
 *
 */
/// applied when a node is inserted in the left subtree of a left subtree
fn rotate_right<K,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
    let mut left: Box<Node<K,D>> = root.left.take().expect("no left child");
    let left_left: Box<Node<K,D>> = left.left.take().expect("no left-left child");

    root.left = left.right;
    left.right = Some(root);
    left
}

/* root                            right
 *     \                          /     \
 *      right    =>           root      right_right
 *          \ 
 *           right_right
 *  move self to self.right.left and return self.right
 */
/// applied when a node is inserted in the right subtree of a right subtree
fn rotate_left<K,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
    let mut right: Box<Node<K,D>> = root.right.take().expect("no right child");
    let right_right: Box<Node<K,D>> = right.right.take().expect("no right-right child");

    root.right = right.left;
    right.left = Some(root);
    right
}

/*
 *      root                   root             left_right
 *     /                      /                 /       \
 *   left           =>      left_right  =>  left        root
 *     \                     / 
 *      left_right        left 
 *
 *  left-rotate left
 *  then right-rotate root
 */
/// applied when a node is inserted in the right subtree of a left subtree
fn rotate_left_right<K,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
    root.left = Some(rotate_left(root.left.expect("no left child")));
    rotate_right(root)
}

/*
 *  root              root                  right_left
 *      \                \                  /       \
 *       right   =>      right_left  =>  root        right
 *      /                     \
 *  right_left                right 
 *
 *  right-rotate right
 *  then left-rotate root
 */
/// applied when a node is inserted in the left subtree of a right subtree
fn rotate_right_left<K,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> {
    root.right = Some(rotate_right(root.right.expect("no right child")));
    rotate_left(root)
}

/// check the balance factor of a subtree rooted at a node and apply any necessary rotations
fn rebalance<K,D>(mut root: Box<Node<K,D>>) -> Box<Node<K,D>> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd {
    match root.balance_factor() {
        -2 => {
            // the sub-tree rooted at this node is left-heavy
            let mut left: Box<Node<K,D>> = root.left.expect("no left node");
            // if the left node is left-heavy, we have a simple rotation
            if (*left).left_heavy() {
                return rotate_right(left);
            } else {
                // left node is right-heavy, do a left-right rotation
                return rotate_left_right(left);
            }
        }
        2 => {
            // the sub-tree rooted at this node is right-heavy
            let mut right: Box<Node<K,D>> = root.right.expect("no right node");
            // if the right node is right-heavy, we have a simple rotation
            if (*right).right_heavy() {
                return rotate_left(right);
            } else {
                // right node is left-heavy, do a right-left rotation
                return rotate_right_left(right);
            }
        }
        _ => return root
    };
}



impl<K,D> PartialEq for Node<K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    fn eq(&self, other: &Self) -> bool {
        (self.key == other.key) && (self.data == other.data)
    }
}


