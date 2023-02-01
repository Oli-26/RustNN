use {
    crate::layer::Layer,
    crate::node::Node,
};

pub struct Network {
    pub name: String,
    pub layers : Vec<Layer>,
}

impl Network {
    pub fn new(name: String, layer_counts: Vec<i32>) -> Self {
        let mut layers: Vec<Layer> = vec![];
        let mut i = 0;
        for count in layer_counts{
            layers.push(Layer::new(i, count));
            i+=1;
        };
        Self {
            name,
            layers,
        }
    }

    pub fn print(self) -> () {
        println!("{} is printing!", self.name);
        let max_layer_width = match self.layers.iter().map(|l| l.nodes.len()).max(){
            Some (count) => count,
            None => 0,
        };

        for layer in self.layers{
            let edge_spaces = max_layer_width - layer.nodes.len();
            
            println!("");
            println!("");
            print!("layer {}     ", layer.depth);
            
            (0..edge_spaces).for_each(|_i| print!("     "));
            layer.nodes.iter().for_each(|_node| print!("Node     "));
        }
    }
}


