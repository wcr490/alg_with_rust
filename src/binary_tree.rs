use std::fmt::Debug;

pub struct BinTree<T>(Option<Box<BinData<T>>>);

pub struct BinData<T> {
    data: T,
    h: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }
    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }
    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }
    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right())
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut self_data) => {
                if self_data.data > data {
                    self_data.left.add_sorted(data);
                    if self_data.left.height() - self_data.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    self_data.right.add_sorted(data);
                    if self_data.right.height() - self_data.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };
        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => (),
        }
        self.set_height();
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_left_first(&self, dp: i32) {
        if let Some(ref self_data) = self.0 {
            self_data.left.print_left_first(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{}: {}{:?}", self.height(), spc, self_data.data);
            self_data.right.print_left_first(dp + 1);
        }
    }
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        let mut right = match self.right.0.take() {
            Some(right) => right,
            None => return Box::new(self),
        };
        self.right = BinTree(right.left.0.take());
        self.right.set_height();
        right.left = BinTree(Some(Box::new(self)));
        right.h = 1 + std::cmp::max(right.left.height(), right.right.height());
        right
    }
    pub fn rot_right(mut self) -> Box<Self> {
        let mut left = match self.left.0.take() {
            Some(left) => left,
            None => return Box::new(self),
        };
        self.left = BinTree(left.right.0.take());
        self.left.set_height();
        left.right.0 = Some(Box::new(self));
        left.h = 1 + std::cmp::max(left.left.height(), left.right.height());
        left
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::BinTree;
    fn example_binary_tree() -> BinTree<i32> {
        let mut bt: BinTree<i32> = BinTree::new();
        bt.add_sorted(2);
        bt.add_sorted(8);
        bt.add_sorted(3);
        bt.add_sorted(5);
        bt.add_sorted(9);
        bt.add_sorted(1);
        bt.add_sorted(6);
        bt
    }

    #[test]
    fn test_binary_tree() {
        let bt: BinTree<i32> = example_binary_tree();
        assert_eq!(
            bt.0.unwrap()
                .right
                .0
                .unwrap()
                .left
                .0
                .unwrap()
                .right
                .0
                .unwrap()
                .right
                .0
                .unwrap()
                .data,
            6
        );
    }
    #[test]
    fn test_binary_tree_left_print() {
        let bt: BinTree<i32> = example_binary_tree();
        bt.print_left_first(0);
        panic!();
    }
    #[test]
    fn test_rot_left() {
        let mut bt: BinTree<i32> = example_binary_tree();
        bt.rot_left();
        bt.print_left_first(0);
        panic!();
    }
    #[test]
    fn test_rot_right() {
        let mut bt: BinTree<i32> = example_binary_tree();
        bt.rot_right();
        bt.print_left_first(0);
        panic!();
    }
}
