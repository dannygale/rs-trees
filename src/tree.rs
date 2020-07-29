use std::cmp;
use std::mem;

use crate::node::Node;

// TODO: use configuration options to handle duplicates
//      (a) put with duplicate key replaces old data
//      (b) put with duplicate key appends data to list in node
//      (c) put with duplicate key keeps data versions (?)
//      (d) ???

type OptBoxNode<K,D> = Option<Box<Node<K,D>>>;


struct NodeIter<'a, K, D> {
    stack: Vec<&'a Node<K,D>>,
    curr: &'a OptBoxNode<K,D>,
}

impl<'a, K,D> Iterator for NodeIter<'a, K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    type Item = &'a Node<K,D>;

    /// iterate over the elements in sorted order
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.curr {
                // if we're at a node 
                Some (ref node) => {
                    // if this node has a left child, save this node on the stack and drill down
                    if node.left.is_some() {
                        self.stack.push(&node);
                        self.curr = &node.left;
                        continue;
                    } 
                    // if this node has a right child, put it on the stack and return this node
                    if node.right.is_some() {
                        self.curr = &node.right;
                        return Some(node);
                    }
                    // return this node and on the next iteration return the one from the top of
                    // the stack
                    // this is kind of like .take() ourself
                    self.curr = &None;
                    return Some(node);
                }

                // we're at a leaf. pop the top node off the stack.
                // if it has a right child, put that on the stack
                // return the popped top node
                None =>  {
                    match self.stack.pop() {
                        Some(node) => {
                            self.curr = &node.right;
                            return Some(node);
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
}



struct AVLTree<K,D> {
    root: OptBoxNode<K,D>
}

impl <'a, K,D> AVLTree<K,D> 
where K: PartialEq + PartialOrd + Copy + Clone,
      D: PartialEq + PartialOrd + Copy + Clone
{
    pub fn new() -> Self {
        Self {
            root: None
        }
    }
    pub fn iter(&'a self) -> NodeIter<'a, K, D> {
        NodeIter {
            stack: Vec::new(),
            curr: &self.root
        }
    }
    pub fn put(&mut self, key: K, data: D) -> bool {
        if let Some(ref mut boxnode) = &mut self.root {
            boxnode.put(key, data);
        } else {
            self.root = Some(Box::new(Node::new(key, data)));
        }
        return true;
    }

    /// return a vector of key/value tuples
    pub fn items(&self) -> Vec<(K,D)> {
        let mut iter = self.iter();
        let mut v = Vec::new();
        loop {
            if let Some(node) = iter.next() {
                v.push((node.key, node.data))
            } else {
                return v;
            }
        }
    }
}

impl<K,D> From <&Vec<(K,D)>> for AVLTree<K,D> 
where K: PartialEq + PartialOrd + Copy + Clone,
      D: PartialEq + PartialOrd + Copy + Clone,
{
    fn from(nodes: &Vec<(K,D)>) -> AVLTree<K,D>{
        let mut tree = AVLTree::new();
        for node in nodes {
            tree.put(node.0, node.1);
        }
        tree
    }
}

use std::iter::{Iterator, FromIterator, IntoIterator};
impl <K,D> FromIterator <Node<K,D>> for AVLTree<K,D> 
where K: PartialEq + PartialOrd + Copy + Clone,
      D: PartialEq + PartialOrd + Copy + Clone, 
{
    fn from_iter<I: IntoIterator<Item = Node<K,D>>>(iter: I) -> Self {
        let mut tree = Self::new();
        for i in iter {
            tree.put(i.key, i.data);
        }
        tree
    }
}

/*
impl<K,D> IntoIterator for AVLTree<K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    type Item = (K,D);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
    }
}
*/

/*
use proptest::prelude::*;

fn arb_node(max_qty: usize) -> impl Strategy<Value = Node<isize, String>> {
    (any::<isize>(), "[a-z]*")
        .prop_map(|(key, data)| Node::new(key, data))
        .boxed()
}

prop_compose! {
    fn arb_node2(_d: isize) 
        (key in 0..100isize, data in "[a-z]*")
            -> Node<isize, String> {
                Node::new(key, data) 
        }
}


proptest! {
    #[test]
    fn test_put_inorder_set(nodes in arb_node2(0)) {
        println!("{:?}", nodes);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_inorder_set() {
        let data = vec![
            (0, "asdf"),
            (1, "qwerty"),
            (2, "zxcv")
        ];
        let tree = AVLTree::from(&data);

        assert!(tree.items().eq(&data));
    }

    #[test]
    fn test_rotate_right() {
        let data = vec![
            (2, "zxcv"),
            (1, "qwerty"),
            (0, "asdf"),
        ];

        let mut a = Node::new(data[0].0, data[0].1);
        let mut b = Node::new(data[1].0, data[1].1); 
        let mut c = Node::new(data[2].0, data[2].1);

        let bref = &mut b;
        let aref = &mut a;

        b.left = Some(Box::new(c));
        a.left = Some(Box::new(b));

        assert_eq!(*bref.left.unwrap(), c);
        assert_eq!(*aref.left.unwrap(), b)
    }
}
