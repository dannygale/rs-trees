use crate::Node;
use std::fmt;

pub enum IterType {
    DFInOrder,
    DFInOrderReverse,
    DFPreOrder,
    DFPostOrder,
    BF
}

pub struct NodeIter<'a, K, D> {
    deque: VecDeque<&'a Box<Node<K,D>>>,
    curr: Option<&'a Box<Node<K, D>>>,
    itype: IterType,
    //next_fn: fn(&mut Self) -> Option<(&K,&D)>
}

impl<'a, K: Ord + Eq, D: Ord + Eq> NodeIter<'a, K, D> {
    pub fn new() -> NodeIter<'a, K, D> {
        NodeIter {
            deque: VecDeque::new(),
            curr: None,
            itype: IterType::DFInOrder
            //next_fn: NodeIter::inorder_next
        }
    }

    pub fn with_root(root: &'a Box<Node<K,D>>) -> NodeIter<'a, K, D> {
        NodeIter {
            deque: VecDeque::new(),
            curr: Some(root),
            itype: IterType::DFInOrder
            //next_fn: NodeIter::inorder_next
        }
    }

    pub fn with_type(self, it: IterType) -> Self {
        NodeIter {
            /*
            next_fn: match it {
                IterType::DFInOrder => Self::inorder_next,
                IterType::DFInOrderReverse => Self::inorder_reversed_next,
                IterType::DFPreOrder => Self::preorder_next,
                IterType::DFPostOrder => Self::postorder_next,
                IterType::BF => Self::bf_next
            },
            */
            itype: it,
            ..self
        }
    }
}

impl<'a, K: Ord + Eq, D: Ord + Eq> NodeIter<'a,K,D> {
    fn inorder_next(&mut self) -> Option<(&'a K,&'a D)> {
        loop {
            match self.curr.take() {
                Some (ref mut node) => {
                    // go left first, if it's there
                    if node.left.is_some() {
                        // save this node so we can come back to it later
                        self.deque.push_back(&node);
                        // drop into the left node
                        self.curr = node.left.as_ref();
                        continue;
                    }

                    // if there's a right child, make sure it's next
                    self.curr = if let Some(right) = &node.right { Some(right) } else {None};
                    // return this node
                    return Some((&node.key, &node.data));
                }

                None => {
                    match self.deque.pop_back() {
                        Some(node) => {
                            self.curr = node.right.as_ref();
                            return Some((&node.key, &node.data));
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
    fn inorder_reversed_next(&mut self) -> Option<(&'a K,&'a D)> {
        // visit right, then self, then left
        loop {
            match self.curr.take() {
                Some (ref mut node) => {
                    // go left first, if it's there
                    if node.right.is_some() {
                        // save this node so we can come back to it later
                        self.deque.push_back(&node);
                        // drop into the left node
                        self.curr = node.right.as_ref();
                        continue;
                    }

                    // if there's a right child, make sure it's next
                    self.curr = if let Some(left) = &node.left { Some(left) } else {None};
                    // return this node
                    return Some((&node.key, &node.data));
                }

                None => {
                    match self.deque.pop_back() {
                        Some(node) => {
                            self.curr = node.left.as_ref();
                            return Some((&node.key, &node.data));
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
    fn preorder_next(&mut self) -> Option<(&'a K,&'a D)> {
        // visit self, then left, then right
        loop {
            match self.curr.take() {
                Some (ref mut node) => {
                    for child in vec![node.right.as_ref(), node.left.as_ref()] {
                        if child.is_some() {
                            self.deque.push_back(&child.unwrap());
                        }
                    }
                    return Some((&node.key, &node.data));
                }

                None => {
                    match self.deque.pop_back() {
                        Some(node) => {
                            return Some((&node.key, &node.data));
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
    fn postorder_next(&mut self) -> Option<(&'a K,&'a D)> {
        // visit left, then right, then self

        loop {
            while let Some(node) = self.curr.take() {
                if let Some(right) = node.right.as_ref() {
                    self.deque.push_back(&right);
                }
                self.deque.push_back(&node);

                self.curr = node.left.as_ref();
            }

            if let Some(node) = self.deque.pop_back() {
                if let Some(right) = node.right.as_ref() {
                    if &right == &self.deque[0] {
                        self.deque.pop_back();
                        self.deque.push_back(node);
                        self.curr = Some(&right);
                    }
                } else {
                    self.curr = None;
                    return Some((&node.key, &node.data));
                }
            } else {
                return None;
            }
        }
    }

    fn bf_next(&mut self) -> Option<(&'a K,&'a D)> {
        // iterate breadth-first -- same as in-order, but with a queue instead of a stack
        loop {
            match self.curr.take() {
                Some (ref mut node) => {
                    if node.left.is_some() {
                        self.deque.push_back(&node);
                        self.curr = node.left.as_ref();
                        continue;
                    }

                    if node.right.is_some() {
                        self.curr = node.right.as_ref();
                        return Some((&node.key, &node.data));
                    }

                    self.curr = None;
                    return Some((&node.key, &node.data));
                }

                None => {
                    match self.deque.pop_front() {
                        Some(node) => {
                            self.curr = node.right.as_ref();
                            return Some((&node.key, &node.data));
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
}

use IterType::*;
impl<'a, K: Ord + Eq, D: Ord + Eq> Iterator for NodeIter<'a,K,D> {
    //type Item = &'a Node<K,D>;
    type Item = (&'a K, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        match self.itype {
            DFInOrder       => { return self.inorder_next() },
            DFInOrderReverse=> { return self.inorder_reversed_next() },
            DFPreOrder      => { return self.preorder_next() },
            DFPostOrder     => { return self.postorder_next() },
            BF              => { return self.bf_next() }
        }

        //return (self.next_fn)(self);
    }
}



use std::collections::vec_deque::VecDeque;

pub struct BreadthIter<'a, K, D> {
    deque: VecDeque<&'a Node<K,D>>,
    curr: Option<&'a Box<Node<K, D>>>
}

impl<'a, K: Ord + Eq, D: Ord + Eq> Iterator for BreadthIter<'a,K,D> {
    //type Item = &'a Node<K,D>;
    type Item = (&'a K, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        // iterate breadth-first -- replace stack from NodeIter with Queue
        loop {
            match self.curr.take() {
                Some (ref mut node) => {
                    if node.left.is_some() {
                        self.deque.push_back(&node);
                        self.curr = node.left.as_ref();
                        continue;
                    }

                    if node.right.is_some() {
                        self.curr = node.right.as_ref();
                        return Some((&node.key, &node.data));
                    }

                    self.curr = None;
                    return Some((&node.key, &node.data));
                }

                None => {
                    match self.deque.pop_back() {
                        Some(node) => {
                            self.curr = node.right.as_ref();
                            return Some((&node.key, &node.data));
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
}

impl<'a, K, D> BreadthIter<'a, K, D> {
    pub fn new() -> BreadthIter<'a, K, D> {
        BreadthIter {
            deque: VecDeque::new(),
            curr: None
        }
    }

    pub fn with_root(root: &'a Box<Node<K,D>>) -> BreadthIter<'a, K, D> {
        BreadthIter {
            deque: VecDeque::new(),
            curr: Some(root)
        }
    }
}


use std::iter::FromIterator;
impl <K,D> FromIterator<(K,D)> for Box<Node<K,D>>
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Ord + Eq + Clone + fmt::Display + fmt::Debug
{
    fn from_iter<I: IntoIterator<Item=(K,D)>>(iter: I) -> Self {
        let mut root: Option<Box<Node<K,D>>> = None;
        for (key, data) in iter {
            if let Some(node) = root {
                root = Some(node.put(key, data));
            } else { root = Some(Node::newbox(key, data)) }
        }
        return root.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;
    use crate::AVLTree;

    #[test]
    fn qc_test_inorder() {
        
    }
    #[test]
    fn test_inorder_reversed() {

    }

    #[test]
    fn test_preorder() {

    }

    #[test]
    fn test_postorder() {
        /* for tree: 
         *          1
         *      2       3
         *    4   5   6   7
         *
         * correct order is: 4526731
         */
        let mut tree = AVLTree::new();
        let vec = vec![1,2,3,4,5,6,7];
        for i in vec {
            tree.put(i, 0);
        }
        let it = NodeIter::with_root(&tree.root.unwrap());

        let ans_vec = vec![(4,0), (5,0), (2,0), (6,0), (7,0), (3,0), (1,0)];

    }

    #[test]
    fn test_breadthfirst() {

    }
}

