use crate::node::{Node, NodeIter};
use std::fmt;

// TODO: use configuration options to handle duplicates
//      (a) put with duplicate key replaces old data
//      (b) put with duplicate key appends data to list in node
//      (c) put with duplicate key keeps data versions (?)
//      (d) ???

type OptBoxNode<K,D> = Option<Box<Node<K,D>>>;

// TODO: Entry API: https://doc.rust-lang.org/std/collections/#entries
pub struct AVLTree<K,D> {
    pub root: OptBoxNode<K,D>
}

impl <'a, K,D> AVLTree<K,D> 
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Ord + Eq + Clone + fmt::Display + fmt::Debug
{
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn with_root(root: Node<K,D>) -> Self {
        let mut tree = AVLTree::new();
        tree.root = Some(Box::new(root));
        return tree;
    }

    pub fn iter(self: &'a Self) -> NodeIter<'a, K, D> {
        self.into_iter()       
    }

    /// insert a new key/data pair into the tree
    pub fn put(&mut self, key: K, data: D) -> bool {
        if self.root.is_some() {
            let root = self.root.take().expect("broken");
            self.root = Some(root.put(key, data));
        } else {
            self.root = Some(Node::newbox(key, data));
        }
        return true;
    }

    /// get a copy of the data associated with a given key
    pub fn get(&self, key: K) -> Option<D> {
        if let Some(root) = self.root.as_ref() {
            if let Some(node) = root.get(key) {
                return Some(node.data.clone());
            } else {
                return None;
            };
        } else { return None }
    }

    /// delete the node specified by key
    pub fn del(&mut self, key: K) -> bool {
        if let Some(root) = self.root.take() {
            if let Ok(node) = root.del(key) {
                self.root = node;
                return true;
            } else {
                return false;
            };
        } else { return false }
    }

    /// return a vector of key/value tuples
    pub fn items(&self) -> Vec<(K,D)> {
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

    pub fn merge(self, other: Self) -> Self {
        match (self.root, other.root) {
            (None, None) => AVLTree::new(),
            (Some(node), None) => AVLTree { root: Some(node) },
            (None, Some(node)) => AVLTree { root: Some(node) },
            (Some(n1), Some(n2)) => AVLTree { root: Some(n1.merge(n2)) }
        }
    }

    pub fn height(&mut self) -> usize {
        if let Some(ref mut root) = self.root {
            return root.height();
        } else { return 0 }
    }
}

impl<K,D> From <&Vec<(K,D)>> for AVLTree<K,D> 
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Ord + Eq + Clone + fmt::Display + fmt::Debug 
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
where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Ord + Eq + Clone + fmt::Display + fmt::Debug
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
//where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Ord + Eq + Clone + fmt::Display + fmt::Debug
//where K: Ord + Eq + Clone + fmt::Display + fmt::Debug, D: Clone + fmt::Display + fmt::Debug
where K: Ord + Eq, D: Ord + Eq
{
    type Item = &'a Node<K,D>;
    type IntoIter = NodeIter<'a, K, D>;

    fn into_iter(self) -> NodeIter<'a, K, D> {
        return NodeIter::with_root(&self.root);
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

    // TODO: test get
    // TODO: test del
    // TODO: test merge
}
