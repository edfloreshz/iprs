// use std::error::Error;
//
// struct Node {
//   pub id: String,
//   nodes: Vec<Node>,
//   pool: Vec<Request>
// }
//
// struct Request {
//   key: String,
// }
//
// impl Node {
//   pub fn new() -> Node {
//     let mut nodes = vec![];
//     // for node in ipfs_network { // TODO: find ipfs network nodes
//     //   nodes.append(node)
//     // }
//     Node {
//       id: "".to_string(), //TODO: Get node ID from ipfs
//       nodes,
//       pool: Vec::new()
//     }
//   }
//   fn send(&mut self, key: String) -> Result<(), Box<dyn Error>> {
//     // for node in self.nodes {
//     //   node
//     // }
//     Ok(())
//   }
// }
