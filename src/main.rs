use crate::network::{Network};
use crate::layer::{Layer};
use crate::node::{Node};

pub mod network;
pub mod layer;
pub mod node;

fn main() {
    println!("Hello, world!");    
    let _network = Network::new("Network 1".to_string(), vec![1,2,3,5,3,2,1]);
    
    _network.print();
}
