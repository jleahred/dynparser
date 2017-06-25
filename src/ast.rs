

// -------------------------------------------------------------------------------------
//  T Y P E S

#[derive(Debug, Clone, Copy)]
pub enum K {
    Root,
    EAnd,
    ENot,
    ERepeat,
    ALit,
    AMatch,
    ADot,
    ASymbref,
    AEof,
}


#[derive(Debug)]
pub struct V(pub String);


#[derive(Debug)]
pub struct Node {
    pub kind: K,
    pub val: V,
    pub nodes: Box<Vec<Node>>,
}

//  T Y P E S
// -------------------------------------------------------------------------------------



impl Node {
    pub fn new(kind: K, val: V) -> Self {
        Node {
            kind: kind,
            val: val,
            nodes: Box::new(vec![]),
        }
    }
    pub fn new_valstr(kind: K, val: &str) -> Self {
        Node {
            kind: kind,
            val: V(val.to_owned()),
            nodes: Box::new(vec![]),
        }
    }
    pub fn merge(mut self, nwnode: Node) -> Self {
        self.nodes.push(nwnode);
        self
    }


    #[must_use]
    pub fn get_pruned<LP>(mut self, lp: &LP) -> Self
        where LP: Fn(&K, &str) -> bool
    {
        let mut located_prune = true;
        while located_prune {
            located_prune = false;
            let mut new_nodes = Box::new(vec![]);
            self.nodes.reverse();
            while let Some(mut node) = self.nodes.pop() {
                match lp(&node.kind, node.val.0.as_ref()) {
                    true => {
                        located_prune = true;
                        node.nodes.reverse();
                        while let Some(n) = node.nodes.pop() {
                            new_nodes.push(n)
                        }
                    }
                    false => new_nodes.push(node.get_pruned(lp)),
                }
            }
            self.nodes = new_nodes;
        }

        self
    }
}
