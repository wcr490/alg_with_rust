/* pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
       match self {
           HuffNode::Tree(l, r) => {
               if let Some(mut v) = l.encode_char(c) {
                   v.insert(0, '0');
                   return Some(v);
               }
               if let Some(mut v) = r.encode_char(c) {
                   v.insert(0, '1');
                   return Some(v);
               }
               None
           }
           HuffNode::Leaf(nc) => {
               if *nc == c {
                   Some(Vec::new())
               } else {
                   None
               }
           }
       }
   }

   pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
       let mut res = Vec::new();
       for c in s.chars() {
           if let Some(v) = self.encode_char(c) {
               res.extend(v.into_iter());
           } else {
               return None;
           }
       }
       Some(res)
   }
*/
/* pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();
    for c in s.chars() {
        let n = *map.get(&c).unwrap_or(&0);
        map.insert(c, n + 1);
    }
    let mut tlist: Vec<HScore> = map
        .into_iter()
        .map(|(k, s)| HScore {
            h: HuffNode::Leaf(k),
            s,
        })
        .collect();

    while tlist.len() > 1 {
        let last = tlist.len() - 1;
        for i in 0..last - 1 {
            if tlist[i].s < tlist[last - 1].s {
                tlist.swap(i, last - 1);
            }
            if tlist[last - 1].s < tlist[last].s {
                tlist.swap(last - 1, last);
            }
        }
        let a_node = tlist.pop().unwrap();
        let b_node = tlist.pop().unwrap();
        let node = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
        tlist.push(HScore {
            h: node,
            s: a_node.s + b_node.s,
        });
    }
    tlist.pop().unwrap().h
} */
use std::{collections::BTreeMap, mem::swap};

#[derive(Debug, PartialEq)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char),
}

pub struct HScore {
    h: HuffNode,
    s: i32,
}

pub fn build_tree(s: &str) -> HuffNode {
    let mut map: BTreeMap<char, i32> = BTreeMap::new();
    for c in s.chars() {
        let ch = *map.get(&c).unwrap_or(&0);
        map.insert(c, ch + 1);
    }
    let mut list: Vec<HScore> = map
        .into_iter()
        .map(|(c, s)| HScore {
            h: HuffNode::Leaf(c),
            s,
        })
        .collect();

    while list.len() > 1 {
        let last = list.len() - 1;
        for index in 0..last - 1 {
            if list[index].s < list[last - 1].s {
                list.swap(index, last - 1);
            }
            if list[last - 1].s < list[last].s {
                list.swap(last - 1, last);
            }
        }
        let a_node = list.pop().unwrap();
        let b_node = list.pop().unwrap();
        let score = HScore {
            h: HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h)),
            s: a_node.s + b_node.s,
        };
        list.push(score);
    }
    list.pop().unwrap().h
}

impl HuffNode {
    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self {
            HuffNode::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}*", spc, dir);
                r.print_lfirst(depth + 1, '\\')
            }
            HuffNode::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}{}", spc, dir, c);
            }
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        match self {
            HuffNode::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            HuffNode::Leaf(leaf_node) => {
                if c == *leaf_node {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }

    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res: Vec<char> = Vec::new();
        for c in s.chars() {
            if let Some(encode_char) = self.encode_char(c) {
                res.extend(encode_char.into_iter());
            } else {
                return None;
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::build_tree;

    #[test]
    fn test_huffman_tree() {
        let s = "hello world hello rust";
        let t = build_tree(s);
        t.print_lfirst(0, '<');
    }

    #[test]
    fn test_huffman_encode() {
        let s = "hello world hello rust";
        let t = build_tree(s);
        println!("encode_char = {:?}", t.encode_str(s));
    }
}
