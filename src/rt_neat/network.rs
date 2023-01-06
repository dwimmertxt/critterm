use itertools::iproduct;
use rand::Rng;
use crate::config;
use crate::rt_neat::speciation;

#[derive(Default, Debug, Clone, Copy)]
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
        Node { id, kind, layer, sum_input, sum_output }
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

impl Connection {
    fn new(
        innovation: usize, unode: usize, vnode: usize,
        weight: f64, enabled: bool) -> Connection {
        Connection { innovation, unode, vnode, weight, enabled }
    }
}

#[derive(Default, Debug)]
pub struct Network {
    nodes:          Vec<Node>,
    connections:    Vec<Connection>,
}

impl Network {
    fn new(nodes: Vec<Node>, connections: Vec<Connection>) -> Network {
        Network { nodes, connections }
    }

    fn establish_connection(&mut self,
        unode: usize, vnode: usize, weight: f64, enabled: bool,
        innovations: &mut speciation::Innovations) {
        //
        let mut new_innov = true;
        let mut cxn = Connection::default();
        for innov in &innovations.id {
            if unode == innov.unode && vnode == innov.vnode {
                cxn = Connection{ 
                    innovation: innov.innovation,
                    unode:      innov.unode,
                    vnode:      innov.vnode,
                    weight,
                    enabled,  
                };
                new_innov = false;
            }
        }
        if new_innov {
            cxn = Connection {
                innovation: innovations.id.len(), unode, vnode, weight, enabled 
            };
            innovations.id.push(cxn);
        }
        self.connections.push(cxn)
    }

    fn mutate(&mut self,
        cfg: &config::Mutation, innovations: &mut speciation::Innovations) {
        //
        let mut rng = rand::thread_rng();
        for cxn in &mut self.connections {
            if rng.gen::<f64>() > cfg.weight.chance {
                continue
            }
            if rng.gen::<f64>() > cfg.weight.adj_threshold {
                cxn.weight = rng.gen_range(-2.0..=2.0);
                continue
            }
            if rng.gen::<f64>() > cfg.weight.add_threshold {
                cxn.weight -= cxn.weight * cfg.weight.sub_factor;
                continue
            }
            cxn.weight += cxn.weight * cfg.weight.add_factor;
        }
        if rng.gen::<f64>() <= cfg.random_connection {
            self.random_connection(cfg, innovations);
        }
        if rng.gen::<f64>() <= cfg.insert_node {
            self.insert_node(innovations);
        }
    }

    fn random_connection(&mut self,
        cfg: &config::Mutation, innovations: &mut speciation::Innovations) {
        //
        let mut rng = rand::thread_rng();
        let nodes_len = self.nodes.len();
        let mut unode = self.nodes[rng.gen_range(0..nodes_len)];
        let mut vnode = self.nodes[rng.gen_range(0..nodes_len)];
        let mut establish_cxn = false;
        // replace 0..n search attempts with pruned list of connection
        // possibilities. only two outcomes: new/toggled connection, or 
        // method safely exits without further ado.
        for _ in 0..20 {
            if unode.id == vnode.id || unode.layer >= vnode.layer {
                unode = self.nodes[rng.gen_range(0..nodes_len)];
                vnode = self.nodes[rng.gen_range(0..nodes_len)];
                continue
            }
            establish_cxn = true;
            for cxn in &mut self.connections {
                if !(cxn.unode == unode.id && cxn.vnode == vnode.id) {   
                    continue
                }
                establish_cxn = false;
                if rng.gen::<f64>() <= 0.25 {
                    match cxn.enabled {
                        true => cxn.enabled = false,
                        false => cxn.enabled = true,
                    }
                }
                break
            }
            break
        }
        if establish_cxn {
            self.establish_connection(
                unode.id, vnode.id, rng.gen_range(-2.0..=2.0),
                true, innovations
            );
        }
    }

    fn insert_node(&mut self, innovations: &mut speciation::Innovations) {
        //
        let mut rng = rand::thread_rng();
        let cxns_len = self.connections.len();
        let mut rand_cxn = self.connections[rng.gen_range(0..cxns_len)];
        rand_cxn.enabled = false;
        let new_node = Node{ 
            id: self.nodes.len(), kind: 0, layer: 0, 
            sum_input: 0, sum_output: 0 };
        self.nodes.push(new_node);

        self.establish_connection( // back half
            rand_cxn.unode, new_node.id, rand_cxn.weight, 
            true, innovations
        );
        self.establish_connection( // forward half
            new_node.id, rand_cxn.vnode, rng.gen_range(-2.0..=2.0), 
            true, innovations
        );
        //self.set_layers();
    }
}


pub fn initialise(
    cfg: &config::RtNeat, 
    innovations: &mut speciation::Innovations) -> Network {
    //
    let mut rng = rand::thread_rng();
    let mut nodes: Vec<Node> = Vec::new();
    let (i, o, h): (usize, usize, usize) = (
        cfg.nodes.input.try_into().unwrap(), 
        cfg.nodes.output.try_into().unwrap(), 
        cfg.nodes.hidden.try_into().unwrap()
    );
    let mut in_out_n: Vec<Node> = Vec::new();
    let mut hidden_n: Vec<Node> = Vec::new();
    for id in 0..i+o+h {
        if id < i {
            nodes.push(Node::new(id, 1, 1, 0, 0));
            in_out_n.push(nodes[id]);
            continue
        }
        if id >= i && id < i + o {
            nodes.push(Node::new(id, 2, 3, 0, 0));
            in_out_n.push(nodes[id]);
            continue
        }
        if id >= i + o {
            nodes.push(Node::new(id, 0, 2, 0, 0));
            hidden_n.push(nodes[id]);
            continue
        }
    }
    let mut network = Network { nodes, connections: Vec::new() };
    while network.connections.is_empty() {
        for (hnode, ionode) in iproduct!(&hidden_n, &in_out_n) {
            if rng.gen::<f64>() < cfg.nodes.connection_chance {
                let (unode, vnode)= match ionode.kind {
                    1 => (ionode.id, hnode.id),
                    2 => (hnode.id, ionode.id),
                    _ => panic!(),
                };
                network.establish_connection(
                    unode, vnode, rng.gen_range(-2.0..=2.0), 
                    true, innovations);
            }
        }
        network.mutate(&cfg.mutation, innovations); 
    }
    network
}

/*
fn dfs(g: HashMap<i32, i32>, v: i32, seen: Vec<i32>, path: Vec<i32>) -> Vec<i32> {
    let mut paths: Vec<i32> = Vec::new();

    if seen.is_empty() {

    }
}
*/
