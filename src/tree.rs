use crate::node::Node;
use std::fmt;

// TODO: use configuration options to handle duplicates
//      (a) put with duplicate key replaces old data
//      (b) put with duplicate key appends data to list in node
//      (c) put with duplicate key keeps data versions (?)
//      (d) ???

type OptBoxNode<K,D> = Option<Box<Node<K,D>>>;



pub struct AVLTree<K,D> {
    root: OptBoxNode<K,D>
}

impl <'a, K,D> AVLTree<K,D> 
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Clone + fmt::Display + fmt::Debug
{
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn iter(self: &'a Self) -> NodeIter<'a, K, D> {
        self.into_iter()       
    }

    pub fn put(&mut self, key: K, data: D) -> bool {
        if self.root.is_some() {
            let root = self.root.take().expect("broken");
            self.root = Some(root.put(key, data));
        } else {
            self.root = Some(Node::newbox(key, data));
        }
        return true;
    }

    pub fn get(&self, key: K) -> Option<&Node<K,D>> {
        if let Some(root) = self.root.as_ref() {
            return root.get(key);
        } else { return None }
    }

    /// return a vector of key/value tuples
    pub fn items(self) -> Vec<(K,D)> {
        let mut iter = self.iter();
        let mut v = Vec::new();
        loop {
            if let Some(node) = iter.next() {
                v.push((node.key.clone(), node.data.clone()))
            } else {
                return v;
            }
        }
    }
}

impl<K,D> From <&Vec<(K,D)>> for AVLTree<K,D> 
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Clone + fmt::Display + fmt::Debug 
{
    fn from(nodes: &Vec<(K,D)>) -> AVLTree<K,D>{
        let mut tree = AVLTree::new();
        for node in nodes {
            tree.put(node.0.clone(), node.1.clone());
        }
        return tree;
    }
}

use std::iter::{Iterator, FromIterator, IntoIterator};

impl <K,D> FromIterator <Node<K,D>> for AVLTree<K,D> 
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Clone + fmt::Display + fmt::Debug
{
    fn from_iter<I: IntoIterator<Item = Node<K,D>>>(iter: I) -> Self {
        let mut tree = Self::new();
        for i in iter {
            tree.put(i.key, i.data);
        }
        return tree;
    }
}

impl <'a, K, D> IntoIterator  for &'a AVLTree<K,D> 
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Clone + fmt::Display + fmt::Debug
{
    type Item = &'a Node<K,D>;
    type IntoIter = NodeIter<'a, K, D>;

    fn into_iter(self) -> NodeIter<'a, K, D> {
        return NodeIter {
            stack: Vec::new(),
            curr: &self.root
        };
    }
}


pub struct NodeIter<'a, K, D> {
    stack: Vec<&'a Node<K,D>>,
    curr: &'a OptBoxNode<K,D>,
}

impl<'a, K,D> Iterator for NodeIter<'a, K,D> 
where K: Ord + Eq
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



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_put_set<K,D> (xs: HashMap<K, D>) 
        where K: Ord + Eq + Clone + fmt::Display + fmt::Debug,
              D: Ord + Eq + Clone + fmt::Display + fmt::Debug
    {
        let mut vec = xs.iter().map(|(x,y)| (x.clone(),y.clone())).collect();
        let tree = AVLTree::from(&vec);
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(tree.items(), vec);
    }

    #[quickcheck]
    fn qc_test_put_set_isize_isize (xs: HashMap<isize, isize>) {
        test_put_set(xs);
    }

    #[quickcheck]
    fn qc_test_put_set_isize_string (xs: HashMap<isize, String>) {
        test_put_set(xs);
    }

    #[quickcheck]
    fn qc_test_put_set_string_string (xs: HashMap<String, String>) {
        test_put_set(xs);
    }
}
