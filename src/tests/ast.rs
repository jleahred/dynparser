#[test]
fn prune_emtpy_root() {
    use ast::{Node, K, V};
    let root = Node::new(K("root".to_owned()), V("".to_owned()));
    let pruned = root.get_pruned();

    assert!(format!("{:?}", pruned) == r#"Node { kind: K("root"), val: V(""), nodes: [] }"#);
}

#[test]
fn prune_nothing_to_prune() {
    use ast::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("child".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };
    let pruned = root.get_pruned();

    assert!(format!("{:?}", pruned) ==
        r#"Node { kind: K("root"), val: V(""), nodes: [Node { kind: K("child"), val: V("aaa"), nodes: [] }] }"#);
}


#[test]
fn prune_repeat_no_child() {
    use ast::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };
    let pruned = root.get_pruned();

    assert!(format!("{:?}", pruned) == r#"Node { kind: K("root"), val: V(""), nodes: [] }"#);
}


#[test]
fn prune_repeat_one_child() {
    use ast::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}


#[test]
fn prune_repeat_two_child() {
    use ast::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K("bbb".to_owned()),
                                                          val: V("bbb".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             },
                             Node {
                                 kind: K("bbb".to_owned()),
                                 val: V("bbb".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}

#[test]
fn prune_repeat_two_child_two_grandchild() {
    use ast::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![Node {
                                                          kind: K("ab".to_owned()),
                                                          val: V("ab".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K("bc".to_owned()),
                                                          val: V("bc".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                                                      },
                                                      Node {
                                                          kind: K("bbb".to_owned()),
                                                          val: V("bbb".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("ab".to_owned()),
                                                          val: V("ab".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K("bc".to_owned()),
                                                          val: V("bc".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             },
                             Node {
                                 kind: K("bbb".to_owned()),
                                 val: V("bbb".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}


#[test]
fn prune_two_nested_repeat_one_child() {
    use ast::{Node, K, V};
    let root =
        Node {
            kind: K("root".to_owned()),
            val: V("".to_owned()),
            nodes: Box::new(vec![Node {
                                     kind: K("repeat".to_owned()),
                                     val: V("".to_owned()),
                                     nodes: Box::new(vec![Node {
                                                              kind: K("repeat".to_owned()),
                                                              val: V("".to_owned()),
                                                              nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                                                          }]),
                                 }]),
        };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}
