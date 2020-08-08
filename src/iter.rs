use crate::Node;
use std::fmt;

/*
enum IterType {
    DFInOrder,
    DFInOrderReverse,
    DFPreOrder,
    DFPostOrder,
    BF
}
*/

pub struct NodeIter<'a, K, D> {
    deque: Vec<&'a Node<K,D>>,
    curr: Option<&'a Box<Node<K, D>>>,
    //itype: IterType
}

impl<'a, K, D> NodeIter<'a, K, D> {
    pub fn new() -> NodeIter<'a, K, D> {
        NodeIter {
            deque: Vec::new(),
            curr: None,
            //itype: IterType::DFInOrder
        }
    }

    pub fn with_root(root: &'a Box<Node<K,D>>) -> NodeIter<'a, K, D> {
        NodeIter {
            deque: Vec::new(),
            curr: Some(root),
            //itype: IterType::DFInOrder
        }
    }

    /*
    pub fn with_type(self, it: IterType) -> Self {
        NodeIter {
            itype: it,
            ..self
        }
    }
    */
}

/*
impl<'a, K: Ord + Eq, D: Ord + Eq> NodeIter<'a,K,D> {
    fn inorder_next(&mut self) -> Option<(K,D)> {
        // visit left, then self, then right
    }
    fn inorder_reversed_next(&mut self) -> Option<(K,D)> {
        // visit right, then self, then left
    }
    fn preorder_next(&mut self) -> Option<(K,D)> {
        // visit self, then left, then right
    }
    fn postorder_next(&mut self) -> Option<(K,D)> {
        // visit left, then right, then self
    }

    fn bf_next(&mut self) -> Option<(K,D)> {
        
    }
}

trait TreeIter<IterType> {
    type Item;

    fn first(&mut self) -> Option<Self::Item>;
    fn second(&mut self) -> Option<Self::Item>;
    fn third(&mut self) -> Option<Self::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.first() { return Some(item) }
        else if let Some(item) = self.second() { return Some(item) }
        else { return self.third() }
    }
}


impl<'a, K: Ord + Eq, D: Ord + Eq> Iterator for NodeIter<'a,K,D> {
    type Item = (K,D);

    fn next(&mut self) -> Option<Self::Item> {
        match self.itype {
            DFInOrder       => { return self.inorder_next() },
            DFInOrderReverse=> { return self.inorder_reversed_next() },
            DFPreOrder      => { return self.preorder_next() },
            DFPostOrder     => { return self.postorder_next() },
            BF              => { return self.bf_next() }
        }
    }
}
*/

impl<'a, K: Ord + Eq, D: Ord + Eq> Iterator for NodeIter<'a,K,D> {
    //type Item = &'a Node<K,D>;
    type Item = (&'a K, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        // iterate in-order -- left, self, right
        loop {
            match self.curr.take() {
                Some (ref mut node) => {
                    // go left first, if it's there
                    if node.left.is_some() {
                        // save this node so we can come back to it later
                        self.deque.push(&node);
                        // drop into the left node
                        self.curr = node.left.as_ref();
                        continue;
                    }

                    // at this point, we've visited all the nodes left of here
                    if node.right.is_some() {
                        // if there's a right node, make sure it's next
                        self.curr = node.right.as_ref();
                        // return this node
                        return Some((&node.key, &node.data));
                    }

                    // so now visit this node
                    self.curr = None;
                    return Some((&node.key, &node.data));
                }

                None => {
                    match self.deque.pop() {
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

