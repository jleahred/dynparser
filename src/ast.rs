

// -------------------------------------------------------------------------------------
//  T Y P E S

#[derive(Debug)]
pub struct K(pub String);

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
    pub fn merge(mut self, nwnode: Node) -> Self {
        self.nodes.push(nwnode);
        self
    }

    #[must_use]
    pub fn get_pruned(mut self) -> Self {
        fn to_be_pruned(node: &Node) -> bool {
            let prune_kind = match node.kind.0.as_ref() {
                "repeat" => true,
                "and" => true,
                // "lit" => true,
                "match" => true,
                "atom" => true,
                "compl_expr" => true,
                _ => false,
            };
            let prune_val = match node.val.0.as_ref() {
                "_" => true,
                "or_expr" => true,
                "and_expr" => true,
                "compl_expr" => true,
                _ => false,
            };
            prune_kind || prune_val
        };


        let mut located_prune = true;
        while located_prune {
            located_prune = false;
            let mut new_nodes = Box::new(vec![]);
            self.nodes.reverse();
            while let Some(mut node) = self.nodes.pop() {
                match to_be_pruned(&node) {
                    true => {
                        located_prune = true;
                        node.nodes.reverse();
                        while let Some(n) = node.nodes.pop() {
                            new_nodes.push(n)
                        }
                    }
                    false => new_nodes.push(node.get_pruned()),
                }
            }
            self.nodes = new_nodes;
        }

        self
    }
}

pub fn from_strs(k: &str, v: &str) -> Node {
    Node::new(K(k.to_owned()), V(v.to_owned()))
}
