use agdb::QueryBuilder;
use fs_graph::Graph;

mod utils;
use utils::*;
use std::path::PathBuf;

/// Test inserting a new node
#[test]
fn create_new_node(){
    let func_name = "create_new_node";
    let mut graph = setup_graph(func_name);

    let path = PathBuf::from("test");
    let alias = PathBuf::from("root/test");

    let node = graph.insert_node_by_path(path, None);

    assert_eq!(node.is_ok(), true);

    let nid = node.unwrap().id;

    let node = graph.db.exec(
        &QueryBuilder::select().ids("root/test").query()
    );

    println!("Node: {:#?}", node);
    assert_eq!(node.is_ok(), true);

    let node = node.unwrap();
    assert_eq!(node.elements.len(), 1);

    let node = node.elements.first().unwrap();
    assert_eq!(node.id, nid);

    // Checking for ntype, nphys, created-time, modified-time
    assert_eq!(node.values.iter().any(|x| x.key == "ntype".into() && x.value == "Other".into()), true);
    assert_eq!(node.values.iter().any(|x| x.key == "nphys".into()), true);
    assert_eq!(node.values.iter().any(|x| x.key == "created_time".into()), true);
    assert_eq!(node.values.iter().any(|x| x.key == "modified_time".into()), true);

    cleanup_graph(&func_name);
}

///
#[test]
fn creating_deep_path_creates_intermediate_nodes() {
    let func_name = "creating_deep_path_creates_intermediate_nodes";
    let mut graph = setup_graph(func_name);

    let path = PathBuf::from("one/two/three");

    let mut node = graph.insert_node_by_path(path, None);

    assert_eq!(node.is_ok(), true);

    cleanup_graph(&func_name);
}

#[test]
fn protect_reserved_node_attributes() {
    let func_name = "protect_reserved_attributes";
    let mut graph = setup_graph(func_name);

    use fs_graph::elements::RESERVED_NODE_ATTRS;

    let path = PathBuf::from("test");
    let node = graph.insert_node_by_path(path, None);

    assert_eq!(node.is_ok(), true);

    cleanup_graph(&func_name);

}
/// Test creating a Node with different NodeTypes
/// Test inserting an existing node (should fail or update)
/// Test opening a node that exists
/// Test opening a node that doesn't exist
/// Test deleting a node
/// Test reparenting a physical node
/// Test reparenting a virtual node
/// Test inserting node attributes (normal and reserved)
/// Test deleting node attributes (normal and reserved)
/// Test merging two nodes
/// Test creating a node with a long path name
/// Test inserting path as alias for a node
/// Test node operations with very deep directory structures
/// Test node operations with many sibling directories/files
/// Test converting NodePath to and from DbValue
/// Test converting NodePhysicality to and from DbValue
/// Test converting NodeType to and from DbValue
#[test]
fn todo_tests() {
    assert_eq!(2 + 2, 4);
}