use itertools::iproduct;
use rand::Rng;
use crate::config;
use crate::rt_neat::{species, mutation};

#[derive(Default, Debug, Clone, Copy)]
pub struct Node {
    pub id:         usize,
    pub kind:       usize,
    pub layer:      usize,
    pub sum_input:  i32,
    pub sum_output: i32,
}

impl Node {
    pub fn new(
        id: usize, kind: usize, layer: usize, 
        sum_input: i32, sum_output: i32) -> Node {
        Node { id, kind, layer, sum_input, sum_output }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Connection {
    innovation: usize,
    pub unode:      usize,
    pub vnode:      usize,
    pub weight:     f64,
    pub enabled:    bool,
}

impl Connection {
    pub fn new(
        innovation: usize, unode: usize, vnode: usize,
        weight: f64, enabled: bool) -> Connection {
        Connection { innovation, unode, vnode, weight, enabled }
    }
}

#[derive(Default, Debug)]
pub struct Network {
    pub nodes:          Vec<Node>,
    pub connections:    Vec<Connection>,
    pub num_layers:     usize,
}

impl Network {
    pub fn new(nodes: Vec<Node>, connections: Vec<Connection>, num_layers: usize) -> Network {
        Network { nodes, connections, num_layers }
    }

    pub fn establish_connection(&mut self,
        unode: usize, vnode: usize, weight: Option<f64>, enabled: bool,
        innovations: &mut species::Innovations) {
        //
        let mut rng = rand::thread_rng();
        let weight = weight.unwrap_or(rng.gen_range(-2.0..=2.0));
        for inv in &innovations.id {
            if !(unode == inv.unode && vnode == inv.vnode) { continue }
            self.connections.push(Connection { 
                innovation: inv.innovation, unode, vnode, weight, enabled  
            });
            return
        }
        let cxn = Connection { innovation: innovations.id.len(), unode, vnode, weight, enabled };
        innovations.id.push(cxn);
        self.connections.push(cxn)
    }

    pub fn insert_node(&mut self,
        node: &Node) {
        //
        self.nodes.push(*node)
    }
}

pub fn init(
    cfg: &config::RtNeat, 
    innovations: &mut species::Innovations) -> Network {
    //
    let mut network = Network { 
        nodes: generate_nodes(&cfg.nodes), connections: Vec::new(), num_layers: 2 
    };
    generate_connections(cfg, innovations, &mut network);
    network
}

fn generate_nodes(cfg: &config::Nodes) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let (i, o) = (cfg.input, cfg.output);
    for id in 0..i+o {
        let (kind, layer) = if id < i { (1, 1) } else { (3, 2) };
        nodes.push(Node::new(id, kind, layer, 0, 0));
    }
    nodes
}

fn generate_connections(
    cfg: &config::RtNeat, innovations: &mut species::Innovations,
    network: &mut Network) {
    //
    connect_inputs_to_outputs(&cfg.nodes.connection_chance, innovations, network);
    for _ in 0..cfg.nodes.hidden { 
        mutation::insert_node(&1.0, innovations, network) 
    }
    mutation::mutate_weights(&cfg.mutation.weight, network);
    mutation::random_connection(&cfg.mutation.random_connection, innovations, network);
    if network.connections.is_empty() { generate_connections(cfg, innovations, network) }
}

fn connect_inputs_to_outputs(
    connection_chance: &f64, innovations: &mut species::Innovations, network: &mut Network) {
    let mut rng = rand::thread_rng();
    for (inode, onode) in iproduct!(
        network.nodes.clone().into_iter().filter(|x| x.kind == 1), 
        network.nodes.clone().into_iter().filter(|x| x.kind == 3) ) {
        if !(rng.gen::<f64>() < *connection_chance) { continue }
        network.establish_connection(inode.id, onode.id, None, true, innovations);
    }
    if network.connections.is_empty() { 
        connect_inputs_to_outputs(connection_chance, innovations, network) 
    }
}







