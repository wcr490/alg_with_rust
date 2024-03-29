use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    rc::{Rc, Weak},
};

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct GraphErr {
    msg: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr { msg: s.to_string() }
    }
}

pub trait Weight {
    fn weight(&self) -> i32;
}

#[derive(Debug)]
pub struct Graph<T, E, ID: Eq + Hash> {
    pub data: HashMap<ID, (T, Vec<ID>)>,
    pub edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Eq + Clone + Hash> Graph<T, E, ID> {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: ID, data: T) {
        self.data.insert(node, (data, Vec::new()));
    }
    pub fn add_edge(&mut self, edge: ID, from: ID, to: ID, edge_data: E) -> Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("fail to find from node"));
        }
        if let Some(ref mut to_data) = self.data.get_mut(&to) {
            self.edges
                .insert(edge, (edge_data, from.clone(), to.clone()));
            to_data.1.push(from.clone());
        } else {
            return Err(GraphErr::new("fail to find to node"));
        }
        self.data.get_mut(&from).unwrap().1.push(to);
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use core::panic;

    use super::{Graph, GraphErr};
    #[test]
    fn test_graph() -> Result<(), GraphErr> {
        let v = vec!['A', 'G', 'R', 'P', 'H'];
        let mut g: Graph<(), (), char> = Graph::new();
        for e in v {
            g.add_node(e, ());
        }
        g.add_edge('a', 'A', 'G', ())?;
        g.add_edge('b', 'R', 'G', ())?;
        println!("graph: {:?}", g);
        Ok(())
    }
}
