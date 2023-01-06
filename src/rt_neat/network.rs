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

#[derive(Default, Debug, Clone, Copy)]
pub struct Connection {
    innovation: usize,
    unode:      usize,
    vnode:      usize,
    weight:     f64,
    enabled:    bool,
}

#[derive(Default, Debug)]
pub struct Graph {
    nodes:          Vec<Node>,
    connections:    Vec<Connection>,
}

impl Graph {
    pub fn new(&mut self,
        cfg: &config::RtNeat, innovations: &mut speciation::Innovations) {
        //
        let mut in_out_nodes: Vec<Node> = Vec::new();
        let mut hidden_nodes: Vec<Node> = Vec::new();

        for _ in 0..cfg.nodes.input {
            self.nodes.push(Node::new(self.nodes.len(), 1, 1, 0, 0));
            in_out_nodes.push(Node::new(self.nodes.len(), 1, 1, 0, 0));
        }
        for _ in 0..cfg.nodes.output {
            self.nodes.push(Node::new(self.nodes.len(), 2, 3, 0, 0));
            in_out_nodes.push(Node::new(self.nodes.len(), 2, 3, 0, 0));
        }
        for _ in 0..cfg.nodes.hidden {
            self.nodes.push(Node::new(self.nodes.len(), 0, 2, 0, 0));
            hidden_nodes.push(Node::new(self.nodes.len(), 0, 2, 0, 0));
        }

        let mut rng = rand::thread_rng();
        while self.connections.len() == 0 {
            for (hnode, ionode) in iproduct!(&hidden_nodes, &in_out_nodes) {
                if rng.gen::<f64>() < cfg.nodes.connection_chance {
                    let (unode, vnode)= match ionode.kind {
                        1 => (ionode.id, hnode.id),
                        2 => (hnode.id, ionode.id),
                        _ => panic!(),
                    };
                    self.connections.push(Graph::establish_connection(
                        unode, vnode, rng.gen_range(-2.0..=2.0), 
                        true, innovations));
                }
            }
            // mutate  
        }
    }

    fn establish_connection(
        unode: usize, vnode: usize, weight: f64, enabled: bool,
        innovations: &mut speciation::Innovations) -> Connection {
        //
        let mut connection = Connection::default();
        connection.weight = weight;
        connection.enabled = enabled;
        for innovation in &innovations.id {
            if unode == innovation.unode && vnode == innovation.vnode {
                connection.innovation = innovation.innovation;
                connection.unode = innovation.unode;
                connection.vnode = innovation.vnode;
                return connection
            }
        }
        connection.innovation = innovations.id.len();
        connection.unode = unode;
        connection.vnode = vnode;
        innovations.id.push(connection);
        return connection
    }

    fn mutate(&self, 
        cfg: config::Mutation, innovations: &mut speciation::Innovations) {
        //
        for connections in &self.connections {
            
        }
    }
}

