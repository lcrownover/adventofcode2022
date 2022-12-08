// use std::{collections::HashMap, hash::Hash};
//
// #[derive(Debug, Clone)]
// enum NodeKind {
//     File,
//     Directory,
// }
//
// #[derive(Debug, Clone)]
// struct Node {
//     id: u32,
//     name: String,
//     kind: NodeKind,
//     size: u32,
//     parent_id: u32,
//     children: HashMap<u32, Node>,
// }
//
// impl Node {
//     fn new_child(
//         &self,
//         id: u32,
//         name: String,
//         kind: NodeKind,
//         size: u32,
//         parent_id: u32,
//         children: HashMap<u32, Node>,
//     ) -> Node {
//         Node {
//             id,
//             name,
//             kind,
//             size,
//             parent_id,
//             children: HashMap::new(),
//         }
//     }
// }
//
// #[derive(Debug)]
// struct Filesystem {
//     nodes: HashMap<u32, Node>,
//     ids: Vec<u32>,
// }
//
// impl Filesystem {
//     fn new() -> Self {
//         Filesystem {
//             nodes: HashMap::new(),
//             ids: Vec::new(),
//         }
//     }
//     fn get_new_id(&self) -> u32 {
//         self.ids.last().unwrap_or(&0.to_owned()) + 1
//     }
//     fn get_node_by_id(&self, id: u32) -> Option<&Node> {
//         self.nodes.get(&id)
//     }
//     fn add_node(
//         &mut self,
//         name: String,
//         kind: NodeKind,
//         size: u32,
//         parent_id: u32,
//         children: HashMap<u32, Node>,
//     ) -> &Node {
//         self.nodes.insert(node.id.to_owned(), node.to_owned());
//     }
// }
//
// fn main() {
//     let mut filesystem = Filesystem::new();
//     filesystem.add_node(node)
//     // println!("{:?}", filesystem);
//     // let mynode = filesystem.new_node("mydir".to_string(), NodeKind::Directory, 0);
//     // println!("{:?}", mynode);
//     // filesystem.add_node(mynode);
//     // println!("{:?}", filesystem);
//     // let mynode2 = filesystem.get_node_by_id(1).unwrap();
//     // println!("{:?}", mynode2);
// }
//
//
//
