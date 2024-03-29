use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use crate::graph::{Graph, Weight};

pub struct Route<ID> {
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}
impl Weight for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

impl<ID: Eq> Route<ID> {
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    pub fn contains(&self, pos: ID) -> bool {
        if self.pos == pos {
            return true;
        }
        match self.path {
            Some(ref path) => path.contains(pos),
            None => false,
        }
    }
}

impl<ID: Debug> Display for Route<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref path) = self.path {
            write!(f, "{}-{}-", path, self.len)?;
        }
        write!(f, "{:?}", self.pos)
    }
}

impl<T, E: Weight, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        let mut visited = HashSet::new();
        let mut routes = Vec::new();
        routes.push(Route::start_rc(from));
        loop {
            let c_route = routes.pop()?;
            if to == c_route.pos {
                return Some(c_route);
            }
            if visited.contains(&c_route.pos) {
                continue;
            }
            visited.insert(c_route.pos.clone());

            let exists = self.data.get(&c_route.pos)?;
            for eid in &exists.1 {
                let edge = self.edges.get(eid)?;
                let npos = if edge.1 == c_route.pos {
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = c_route.len + edge.0.weight();
                let nroute = Rc::new(Route {
                    pos: npos,
                    len: nlen,
                    path: Some(c_route.clone()),
                });
                if routes.len() == 0 {
                    routes.push(nroute.clone());
                    continue;
                }
                let mut iafter = routes.len() - 1;
                loop {
                    if routes[iafter].len > nlen {
                        routes.insert(iafter + 1, nroute.clone());
                        break;
                    }
                    if iafter == 0 {
                        routes.insert(0, nroute);
                        break;
                    }
                    iafter -= 1;
                }
            }
        }
    }
}
