
use ast::{Node, K, V};

fn check2prune(kind: &K, val: &str) -> bool {
    let prune_kind = match kind {
        &K::ERepeat => true,
        _ => false,
    };
    let prune_val = match val {
        _ => false,
    };
    prune_kind || prune_val

}


#[test]
fn prune_emtpy_root() {
    let root = Node::new_valstr(K::Root, "");
    let pruned = root.get_pruned(&check2prune);

    assert!(format!("{:?}", pruned) == r#"Node { kind: Root, val: V(""), nodes: [] }"#);
}

#[test]
fn prune_nothing_to_prune() {
    let root = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ALit,
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };
    let pruned = root.get_pruned(&check2prune);

    assert!(format!("{:?}", pruned) ==
        r#"Node { kind: Root, val: V(""), nodes: [Node { kind: ALit, val: V("aaa"), nodes: [] }] }"#);
}


#[test]
fn prune_repeat_no_child() {
    let root = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ERepeat,
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };
    let pruned = root.get_pruned(&check2prune);

    assert!(format!("{:?}", pruned) == r#"Node { kind: Root, val: V(""), nodes: [] }"#);
}


#[test]
fn prune_repeat_one_child() {
    let root = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ERepeat,
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K::ALit,
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned(&check2prune);

    let result = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ALit,
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}


#[test]
fn prune_repeat_two_child() {
    let root = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ERepeat,
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K::ALit,
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K::ALit,
                                                          val: V("bbb".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned(&check2prune);

    let result = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ALit,
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             },
                             Node {
                                 kind: K::ALit,
                                 val: V("bbb".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}

#[test]
fn prune_repeat_two_child_two_grandchild() {
    let root = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ERepeat,
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K::ALit,
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![Node {
                                                          kind: K::ALit,
                                                          val: V("ab".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K::ALit,
                                                          val: V("bc".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                                                      },
                                                      Node {
                                                          kind: K::ALit,
                                                          val: V("bbb".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned(&check2prune);

    let result = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ALit,
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K::ALit,
                                                          val: V("ab".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K::ALit,
                                                          val: V("bc".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             },
                             Node {
                                 kind: K::ALit,
                                 val: V("bbb".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}


#[test]
fn prune_two_nested_repeat_one_child() {
    let root =
        Node {
            kind: K::Root,
            val: V("".to_owned()),
            nodes: Box::new(vec![Node {
                                     kind: K::ERepeat,
                                     val: V("".to_owned()),
                                     nodes: Box::new(vec![Node {
                                                              kind: K::ERepeat,
                                                              val: V("".to_owned()),
                                                              nodes: Box::new(vec![Node {
                                                            kind: K::ALit,
                                                            val: V("aaa".to_owned()),
                                                            nodes: Box::new(vec![]),
                                                      }]),
                                                          }]),
                                 }]),
        };
    let pruned = root.get_pruned(&check2prune);

    let result = Node {
        kind: K::Root,
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K::ALit,
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}
