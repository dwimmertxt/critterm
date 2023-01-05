use itertools::iproduct;
use rand::Rng;
use crate::config;
use crate::rt_neat::speciation;

#[derive(Default, Debug, Clone)]
struct Node {
    id:         usize,
    kind:       i32,
    layer:      i32,
    sum_input:  i32,
    sum_output: i32,
}

impl Node {
    fn new(
        id: usize, kind: i32, layer: i32, 
        sum_input: i32, sum_output: i32) -> Node {
        Node {
            id: id, kind: kind, layer: layer,
            sum_input: sum_input, sum_output: sum_output
        }
    }
}

#[derive(Default, Debug)]
pub struct Connection {
    innovation: i32,
    u_node:     i32,
    v_node:     i32,
    weight:     i32,
    enabled:    bool,
}

#[derive(Default, Debug)]
pub struct Graph {
    nodes:          Vec<Node>,
    connections:    Vec<Connection>,
}

impl Graph {
    pub fn new(&mut self,
        cfg: &config::Config, innovations: &speciation::Innovations) {
        //
        let mut in_out_nodes: Vec<Node> = Vec::new();
        let mut hidden_nodes: Vec<Node> = Vec::new();

        for i in 0..cfg.critters.nodes.input {
            self.nodes.push(Node::new(self.nodes.len(), 1, 1, 0, 0));
            in_out_nodes.push(Node::new(self.nodes.len(), 1, 1, 0, 0));
        }
        for i in 0..cfg.critters.nodes.output {
            self.nodes.push(Node::new(self.nodes.len(), 2, 3, 0, 0));
            in_out_nodes.push(Node::new(self.nodes.len(), 2, 3, 0, 0));
        }
        for i in 0..cfg.critters.nodes.hidden {
            self.nodes.push(Node::new(self.nodes.len(), 0, 2, 0, 0));
            hidden_nodes.push(Node::new(self.nodes.len(), 0, 2, 0, 0));
        }

        let mut rng = rand::thread_rng();
        for (hn, ion) in iproduct!(hidden_nodes, in_out_nodes) {
            if rng.gen::<f64>() < cfg.critters.connection_chance {
                let (mut un, mut vn)= match ion.kind {
                    1 => (ion.id, hn.id),
                    2 => (hn.id, ion.id),
                    _ => panic!(),
                };
                //g.establish_connection();
            }
        }
    }
    //fn establish_connection()
}

