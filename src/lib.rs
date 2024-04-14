#![allow(non_snake_case)]

// use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, rc::Rc};

type pointer = Option<Node>;
#[derive(Debug)]
pub struct Node {
    element: i32,
}

impl Node {
    fn new(element: &i32) -> Self {
        Node { element: *element }
    }
}

#[derive(Debug)]
pub struct BinaryTree {
    root: Option<Node>,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn init(rootNode: Option<Node>) -> Self {
        BinaryTree {
            root: rootNode,
            left: None,
            right: None,
        }
    }

    pub fn initFromElement(element: &i32) -> Self {
        BinaryTree {
            root: Some(Node { element: *element }),
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, element: &i32) {
        let newNode = Node::new(element);

        match self.root {
            None => self.root = Some(newNode),
            Some(ref mut rootNode) => {
                if rootNode.element < *element {
                    match self.right {
                        None => self.right = Some(Box::new(BinaryTree::init(Some(newNode)))),
                        Some(ref mut rightTree) => rightTree.add(element),
                    }
                } else {
                    match self.left {
                        None => self.left = Some(Box::new(BinaryTree::init(Some(newNode)))),
                        Some(ref mut leftTree) => leftTree.add(element),
                    }
                }
            }
        }
    }

    pub fn pricesInRange<'a>(&self, start: &i32, end: &i32, prices: &'a mut Vec<i32>) -> &'a mut Vec<i32> {
        match self.root {
            None => prices,
            Some(ref rootTree) => {
                if rootTree.element > *start {
                    if rootTree.element < *end {
                        prices.push(rootTree.element);
                    }
                    
                    match self.left {
                        None => prices,
                        Some(ref leftTree) => leftTree.pricesInRange(start, end, prices)
                    }
                } else {
                    match self.right {
                        None => prices,
                        Some(ref rightTree) => rightTree.pricesInRange(start, end, prices)
                    }
                }
            }
        }
    }
}
