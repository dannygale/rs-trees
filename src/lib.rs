#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod node;
pub use node::Node;

mod tree;
pub use tree::AVLTree;

