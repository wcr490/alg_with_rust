use std::cell::RefCell;
use std::fmt::{Debug, Write};
use std::rc::Rc;

use crate::b_rand::rand;

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

#[derive(Debug)]
pub struct SkipList<T: PartialOrd>(Vec<SkipNode<T>>);

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }
    pub fn insert(&mut self, data: T) -> Option<Rcc<SkipNode<T>>> {
        if let Some(ref right) = self.right {
            if data > *right.borrow().data.borrow() {
                return right.borrow_mut().insert(data);
            }
        }
        match self.down {
            Some(ref child) => {
                if rand::random::<bool>() {
                    let node = SkipNode {
                        right: self.right.take(),
                        down: Some(child.clone()),
                        data: child.borrow().data.clone(),
                    };
                    let res = rcc(node);
                    self.right = Some(res.clone());
                    Some(res)
                } else {
                    None
                }
            }
            None => None,
        };
        let node = SkipNode {
            right: self.right.take(),
            down: None,
            data: rcc(data),
        };
        let ret = rcc(node);
        self.right = Some(ret.clone());
        Some(ret)
    }
}

impl<T: PartialOrd> SkipList<T> {
    fn new() -> Self {
        SkipList(Vec::new())
    }
    pub fn insert(&mut self, data: T) {
        if self.0.len() == 0 {
            self.0.push(SkipNode::new(data));
            return;
        }
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(child) = self.0[i].insert(data) {
                    self.loop_up(child, i + 1);
                }
                return;
            }
        }
        let mut node = SkipNode::new(data);
        std::mem::swap(&mut node, &mut self.0[0]);
        let ret = rcc(node);
        self.0[0].right = Some(ret.clone());
        self.loop_up(ret, 1);
    }
    pub fn loop_up(&mut self, ch: Rcc<SkipNode<T>>, n: usize) {
        if rand::random::<bool>() {
            return;
        }
        let data = ch.borrow().data.clone();
        let mut node = SkipNode {
            right: None,
            down: Some(ch),
            data,
        };
        if n >= self.0.len() {
            self.0.push(node);
            return;
        }
        std::mem::swap(&mut node, &mut self.0[n]);
        let ret = rcc(node);
        self.0[n].right = Some(ret.clone());
        self.loop_up(ret, n + 1);
    }
}

impl<T: PartialOrd + Debug> SkipNode<T> {
    pub fn print_row<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:?}", self.data.borrow());
        if let Some(ref r) = self.right {
            write!(w, ",")?;
            r.borrow().print_row(w);
        }
        Ok(())
    }
}

impl<T: PartialOrd + Debug> std::fmt::Display for SkipList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "SkipNode<Empty>");
        }
        for sn in &self.0 {
            write!(f, "\n")?;
            sn.print_row(f)?;
        }
        Ok(())
    }
}
