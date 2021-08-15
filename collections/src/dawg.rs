use std::ops::BitOr;

use bitvec::{BitArr, index::BitMask};

pub struct Dawg {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Dawg {
    pub fn new() -> Self {
        let nodes = vec![Node::new()];
        let edges = vec![];
        Self { nodes, edges }
    }

    pub fn root(&self) -> &Node {
        self.nodes.first().unwrap()
    }

    pub fn leaf(&self) -> &Node {
        self.nodes.last().unwrap()
    }
}

struct Node {
    children: BitMask<>
}

impl Node {
    pub fn new() -> Self {
        Self { children: U256::zero() }
    }

    pub fn add(&mut self, child: u8) -> bool {
        let i = child as usize;
        if self.children.bit(i) {
            return false;
        }
        self.children = self.children | (U256::one() << child as usize);
        true
    }

    pub fn remove(&mut self, child: u8) -> bool {
        let i = child as usize;
        if !self.children.bit(i) {
            return false;
        }
        todo!();
        true
    }
}

struct Edge {
    from: usize,
    to: usize,
    key: u8,
}