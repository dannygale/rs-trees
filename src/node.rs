use std::cmp;
use cmp::Ordering::{Equal,Greater,Less};
use std::fmt;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

type OptBoxNode<K,D> = Option<Box<Node<K,D>>>;

#[derive(Default)]
pub struct Node<K, D> {
    pub key: K,
    pub data: D,

    pub height: isize,

    pub left: OptBoxNode<K,D>,
    pub right: OptBoxNode<K,D>,
}

impl<K: fmt::Debug, D: fmt::Debug> fmt::Debug for Node<K,D> {
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

impl<K: fmt::Debug + fmt::Display, D: fmt::Debug + fmt::Display> fmt::Display for Node<K,D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = match &self.left {
            Some(node) => format!("Node {{ {}:{} }}", node.key, node.data),
            None => String::from("None"),
        };
        let right = match &self.right {
            Some(node) => format!("Node {{ {}:{} }}", node.key, node.data),
            None => String::from("None"),
        };
        write!(f, "{{ {}:{}, left: {}, right: {} }}", &self.key, &self.data, left, right)
    }
}

impl<K: fmt::Display + fmt::Debug + Eq + Ord, D: fmt::Display + fmt::Debug> Node<K,D>  {
    pub fn new(key: K, data: D) -> Self {
        return Self { key, data, height: 0, left: None, right: None };
    }

    pub fn newbox(key: K, data: D) -> Box<Self> {
        return Box::new(Self::new(key, data));
    }

    fn height(&mut self) -> isize {
        // cache result from potentially expensive drill-down
        // TODO: when does this need to be invalidated?
        /*
        if self.height != 0 {
            return self.height;
        }
        */

        return self.update_height();
    }

    fn update_height(&mut self) -> isize {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(node) = &mut self.left {
            left_height = node.height();
        } 
        if let Some(node) = &mut self.right {
            right_height = node.height();
        } 

        self.height = cmp::max(left_height, right_height) + 1;
        return self.height;
    }

    /// return the difference in height between the right tree and the left tree
    /// a positive value indicates that the right tree is deeper
    /// a negative value indicates that the left tree is deeper
    pub fn balance_factor(&mut self) -> isize {
        return self.right_height() - self.left_height();
    }

    pub fn left_heavy(&mut self) -> bool {
        self.balance_factor() < 0
    }
    pub fn right_heavy(&mut self) -> bool {
        self.balance_factor() > 0
    }

    fn right_height(&mut self) -> isize {
        return match &mut self.right {
            Some(node) => node.height(),
            None => 0
        };
    }
    fn left_height(&mut self) -> isize {
        return match &mut self.left {
            Some(node) => node.height(),
            None => 0
        };
    }

    /// recursively search for the given key
    pub fn get(&self, key: K) -> Option<&Node<K,D>> {
        debug!("searching for key '{}'", key);
        if key == self.key {
            return Some(&self);
        } else if key < self.key {
            if let Some(node) = &self.left {
                return node.get(key);
            } else { 
                return None; 
            }
        } else { // key > self.key
            if let Some(node) = &self.right {
                return node.get(key);
            } else { 
                return None; 
            }
        }
    }

    /// insert a new key/data pair
    pub fn put(mut self: Box<Self>, key: K, data: D) -> Box<Self> {
        self.height = 0;
        trace!("put {}:{} into self: {}", key, data, &*self);

        match self.key.cmp(&key) {
            Equal => {
                trace!("{} == {}, replacing data", self.key, key);
                self.data = data;
                return self;
            },
            Greater => {
                let l = self.left.take();
                trace!("new key {} < self.key {}, putting in left child", key, self.key);
                self.left = self.put_in_child(key, data, l);
            }
            Less => { 
                let r = self.right.take();
                trace!("new key {} > self.key {}, putting in right child", key, self.key);
                self.right = self.put_in_child(key, data, r);
            }
        };

        return self.rebalance();
    }

    fn put_in_child(&mut self, key: K, data: D, child: OptBoxNode<K,D>) -> OptBoxNode<K,D> {
        Some(
            match child {
                Some(node) => node.put(key, data),
                None => {
                    trace!("creating new node");
                    Node::newbox(key, data)
                }
            }
        )
    }

    /* right rotation after a node is inserted in the left subtree of a left subtree
     * left rotation after a node is inserted in the right subtree of a right subtree
     * left-right rotation after a node is inserted as the right subtree of a left subtree
     * right-left rotation after a node is inserted as the left subtree of a right subtree
     * ref: https://www.educative.io/edpresso/common-avl-rotation-techniques
     */
    /// check the balance factor of a subtree rooted at a node and apply any necessary rotations
    fn rebalance(mut self: Box<Self>) -> Box<Node<K,D>> 
    where K: Eq + Ord, {
        let bf = self.balance_factor();
        trace!("balance factor {} for {}", &bf, &self);
        match bf {
            -2 => {
                // the sub-tree rooted at this node is left-heavy
                let left: &mut Box<Node<K,D>> = self.left.as_mut().expect("no left node");
                // if the left node is left-heavy, we have a simple rotation
                if left.left_heavy() {
                    trace!("left node is left heavy: left = {}", &left);
                    return self.rotate_right();
                } else {
                    // left node is right-heavy, do a left-right rotation
                    trace!("left node is right heavy: left = {}", &left);
                    return self.rotate_left_right();
                }
            }
            2 => {
                // the sub-tree rooted at this node is right-heavy
                let right: &mut Box<Node<K,D>> = self.right.as_mut().expect("no right node");
                // if the right node is right-heavy, we have a simple rotation
                if right.right_heavy() {
                    trace!("right node is right heavy: right = {}", &right);
                    return self.rotate_left();
                } else {
                    // right node is left-heavy, do a right-left rotation
                    trace!("right node is left heavy: right = {}", &right);
                    return self.rotate_right_left();
                }
            }
            _ => return self
        };
    }



    /*
     *             root                  left
     *            /                     /    \
     *           left    =>     left_left    root
     *          /
     * left_left
     *
     */
     /// applied when a node is inserted in the left subtree of a left subtree

    fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        trace!("rotate_right: {}", self);
        let mut left: Box<Node<K,D>> = self.left.take().expect("no left child");
        //let left_left: Box<Node<K,D>> = left.left.take().expect("no left-left child");

        self.left = left.right;
        left.right = Some(self);
        return left;
    }

    /* root                           right
     *     \                          /     \
     *      right    =>           root      right_right
     *          \ 
     *           right_right
     * move root to root.right.left and return root.right
     */
     /// applied when a node is inserted in the right subtree of a right subtree
    fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        trace!("rotate_left: {}", self);
        let mut right: Box<Node<K,D>> = self.right.take().expect("no right child");
        trace!("rotate_left: right_child: {}", &right);
        //let right_right: Box<Node<K,D>> = right.right.take().expect("no right-right child");

        self.right = right.left;
        right.left = Some(self);
        right
    }

    /*
     *     root                   root             left_right
     *    /                      /                 /       \
     *  left           =>      left_right  =>  left        root
     *    \                     / 
     *     left_right        left 
     *
     * left-rotate left
     * then right-rotate root
     */
     /// applied when a node is inserted in the right subtree of a left subtree
    fn rotate_left_right(mut self: Box<Self>) -> Box<Self> {
        trace!("rotate_left_right: {}", self);
        self.left = Some(self.left.expect("no left child").rotate_left());
        return self.rotate_right();
    }

    /*
     * root              root                  right_left
     *     \                \                  /       \
     *      right   =>      right_left  =>  root        right
     *     /                     \
     * right_left                right 
     *
     * right-rotate right
     * then left-rotate root
     *
     */
     /// applied when a node is inserted in the left subtree of a right subtree
    fn rotate_right_left(mut self: Box<Self>) -> Box<Self> {
        trace!("rotate_right_left: {}", self);
        self.right = Some(self.right.expect("no right child").rotate_right());
        return self.rotate_left();
    }

    /// in a node with two children, in-order predecessor is right-most child of left subtree
    fn in_order_pred(&self) -> &Box<Self> {
        let mut node: &Box<Self> = self.left.as_ref().expect("no left child");
        while let Some(next) = node.right.as_ref() { node = next };
        return node;
    }

    /// in a node with two children, in-order successor is left-most child of right subtree
    fn in_order_succ(&self) -> &Box<Self> {
        let mut node: &Box<Self> = self.right.as_ref().expect("no right child");
        while let Some(next) = node.left.as_ref() { node = next };
        return node;
    }

}


impl<K: Ord + Eq,D: Ord + Eq> PartialEq for Node<K,D>  {
    fn eq(&self, other: &Self) -> bool {
        (self.key == other.key) && (self.data == other.data)
    }
}

impl<K: Ord + Eq, D: Ord + Eq> Eq for Node<K,D> {  }


impl<K: Ord + Eq,D: Ord + Eq> Ord for Node<K,D>  {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return (&self.key, &self.data).cmp(&(&other.key, &other.data));
    }
}

impl<K: Ord + Eq,D: Ord + Eq> PartialOrd for Node<K,D>  {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    fn test_ordering<K: Ord+Eq+fmt::Display+fmt::Debug+Clone, D: Ord+Eq+fmt::Display+fmt::Debug+Clone>(first: (K,D), second: (K,D)) {
        let n1 = Node::new(first.0.clone(), first.1.clone());
        let n2 = Node::new(second.0.clone(), second.1.clone());

        match n1.key.cmp(&n2.key) {
            // if keys are equal, make sure we are sorting based on data
            Equal => assert_eq!(first.1.cmp(&second.1), n1.cmp(&n2)),
            // if keys are not equal, ensure node order matches key order
            Less => assert_eq!(n1.cmp(&n2), Less),
            Greater => assert_eq!(n1.cmp(&n2), Greater)
        }
    }

    #[quickcheck]
    fn qc_test_ordering_isize_isize(n1: (isize, isize), n2: (isize, isize)) {
        test_ordering(n1, n2);
    }

    #[quickcheck]
    fn qc_test_ordering_isize_string(n1: (isize, String), n2: (isize, String)) {
        test_ordering(n1, n2);
    }

    #[quickcheck]
    fn qc_test_ordering_string_string(n1: (String, String), n2: (String, String)) {
        test_ordering(n1, n2);
    }

    #[test]
    fn test_balance_factor () {
        let mut root = Node::newbox(2, "root");
        let mut left = Node::newbox(1, "left");
        let left_left = Node::newbox(0, "left_left");

        left.left = Some(left_left);
        left.update_height();
        assert_eq!(left.height, 2);
        assert_eq!(left.as_mut().balance_factor(), -1);
        assert_eq!(left.as_mut().left_heavy(), true);

        root.left = Some(left);
        root.update_height();

        assert_eq!(root.balance_factor(), -2);
        assert!(root.left_heavy());
    }

    #[test]
    fn test_rotate_right () {
        let mut root = Node::newbox(2isize, "asdf");
        let mut left = Node::newbox(1isize, "qwerty");
        let left_left = Node::newbox(0isize, "zxcv");

        left.left = Some(left_left);
        root.left = Some(left);

        assert_eq!(&root.right, &None);
        assert_eq!(root.left.as_ref().unwrap(), &Node::newbox(1, "qwerty"));
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap(), &Node::newbox(0, "zxcv"));

        let new_root = root.rotate_right();

        assert_eq!(new_root, Node::newbox(1isize, "qwerty"));
        assert_eq!(new_root.right.unwrap(), Node::newbox(2,"asdf"));
        assert_eq!(new_root.left.unwrap(), Node::newbox(0isize,"zxcv"));
    }

    #[test]
    fn test_rotate_left () {
        let mut root = Node::newbox(2isize, "root");
        let mut right = Node::newbox(1isize, "right");
        let right_right = Node::newbox(0isize, "right_right");

        right.right= Some(right_right);
        root.right = Some(right);

        assert_eq!(&root.left, &None);
        assert_eq!(root.right.as_ref().unwrap(), &Node::newbox(1, "right"));
        assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap(), &Node::newbox(0, "right_right"));

        let new_root = root.rotate_left();

        assert_eq!(new_root, Node::newbox(1isize, "right"));
        assert_eq!(new_root.left.unwrap(), Node::newbox(2,"root"));
        assert_eq!(new_root.right.unwrap(), Node::newbox(0isize,"right_right"));
    }

    #[test]
    fn test_rotate_right_left () {
        let mut root = Node::newbox(2, "root");
        let mut right = Node::newbox(1, "right");
        let right_left = Node::newbox(0, "right_left");

        right.left = Some(right_left);
        root.right = Some(right);

        assert_eq!(&root.left, &None);
        assert_eq!(root.right.as_ref().unwrap(), &Node::newbox(1, "right"));
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap(), &Node::newbox(0, "right_left"));

        let new_root = root.rotate_right_left();

        assert_eq!(new_root, Node::newbox(0, "right_left"));
        assert_eq!(new_root.left.unwrap(), Node::newbox(2,"root"));
        assert_eq!(new_root.right.unwrap(), Node::newbox(1,"right"));
    }

    #[test]
    fn test_rotate_left_right () {
        let mut root = Node::newbox(2, "root");
        let mut left = Node::newbox(1, "left");
        let left_right = Node::newbox(0, "left_right");

        left.right = Some(left_right);
        root.left = Some(left);

        assert_eq!(&root.right, &None);
        assert_eq!(root.left.as_ref().unwrap(), &Node::newbox(1, "left"));
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap(), &Node::newbox(0, "left_right"));

        let new_root = root.rotate_left_right();

        assert_eq!(new_root, Node::newbox(0, "left_right"));
        assert_eq!(new_root.right.unwrap(), Node::newbox(2,"root"));
        assert_eq!(new_root.left.unwrap(), Node::newbox(1,"left"));
    }

    #[test]
    fn test_put () {
        let mut root = Node::newbox(0, 0);
        root = root.put(1,1);

        assert_eq!(root.right, Some(Node::newbox(1,1)));
        assert_eq!(root.left, None);

        root = root.put(2,2);
        trace!("{}", &root);
        trace!("{}", root.left.as_ref().unwrap());
        assert_eq!(root, Node::newbox(1,1));

    }
}

