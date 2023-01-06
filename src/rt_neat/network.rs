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
        Node {
            id, kind, layer,
            sum_input, sum_output
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
    pub fn new(
        cfg: &config::RtNeat, 
        innovations: &mut speciation::Innovations) -> Graph {
        //
        let mut nodes: Vec<Node> = Vec::new();
        let mut in_out_nodes: Vec<Node> = Vec::new();
        let mut hidden_nodes: Vec<Node> = Vec::new();
        for _ in 0..cfg.nodes.input {
            nodes.push(Node::new(nodes.len(), 1, 1, 0, 0));
            in_out_nodes.push(Node::new(nodes.len(), 1, 1, 0, 0));
        }
        for _ in 0..cfg.nodes.output {
            nodes.push(Node::new(nodes.len(), 2, 3, 0, 0));
            in_out_nodes.push(Node::new(nodes.len(), 2, 3, 0, 0));
        }
        for _ in 0..cfg.nodes.hidden {
            nodes.push(Node::new(nodes.len(), 0, 2, 0, 0));
            hidden_nodes.push(Node::new(nodes.len(), 0, 2, 0, 0));
        }
        let mut connections: Vec<Connection> = Vec::new();
        let mut rng = rand::thread_rng();
        while connections.is_empty() {
            for (hnode, ionode) in iproduct!(&hidden_nodes, &in_out_nodes) {
                if rng.gen::<f64>() < cfg.nodes.connection_chance {
                    let (unode, vnode)= match ionode.kind {
                        1 => (ionode.id, hnode.id),
                        2 => (hnode.id, ionode.id),
                        _ => panic!(),
                    };
                    connections.push(Graph::establish_connection(
                        unode, vnode, rng.gen_range(-2.0..=2.0), 
                        true, innovations));
                }
            }
            Graph::mutate(&cfg.mutation, innovations, &mut nodes, &mut connections); 
        }
        Graph { nodes, connections }
    }

    fn establish_connection(
        unode: usize, vnode: usize, weight: f64, enabled: bool,
        innovations: &mut speciation::Innovations) -> Connection {
        //
        for innovation in &innovations.id {
            if unode == innovation.unode && vnode == innovation.vnode {
                return Connection{ 
                    innovation: innovation.innovation,
                    unode:      innovation.unode,
                    vnode:      innovation.vnode,
                    weight,
                    enabled,  
                }
            }
        }
        let connection = Connection {
            innovation: innovations.id.len(), unode, vnode, weight, enabled,
        };
        innovations.id.push(connection);
        connection
    }

    fn mutate( 
        cfg: &config::Mutation, innovations: &mut speciation::Innovations,
        nodes: &mut Vec<Node>, connections: &mut Vec<Connection>) {
        //
        let mut rng = rand::thread_rng();
        for connection in &mut *connections {
            if rng.gen::<f64>() > cfg.weight.chance {
                continue
            }
            if rng.gen::<f64>() > cfg.weight.adj_threshold {
                connection.weight = rng.gen_range(-2.0..=2.0);
                continue
            }
            if rng.gen::<f64>() > cfg.weight.add_threshold {
                connection.weight -= connection.weight * cfg.weight.sub_factor;
                continue
            }
            connection.weight += connection.weight * cfg.weight.add_factor;
        }
        if rng.gen::<f64>() <= cfg.random_connection {
            Graph::random_connection(cfg, innovations, nodes, connections);
        }
        if rng.gen::<f64>() <= cfg.new_node {
            Graph::new_node(innovations, &mut *nodes, connections);
        }
    }

    fn random_connection(
        cfg: &config::Mutation, innovations: &mut speciation::Innovations,
        nodes: &Vec<Node>, connections: &mut Vec<Connection>) {
        //
        let mut rng = rand::thread_rng();
        let mut unode = &nodes[rng.gen_range(0..nodes.len())];
        let mut vnode = &nodes[rng.gen_range(0..nodes.len())];
        let mut establish_connection = true;
        for _ in 0..20 {
            unode = &nodes[rng.gen_range(0..nodes.len())];
            vnode = &nodes[rng.gen_range(0..nodes.len())];
            if unode.id == vnode.id || unode.layer >= vnode.layer {
                continue
            }
            for connection in &mut *connections {
                if !(connection.unode == unode.id && connection.vnode == vnode.id) {
                    continue
                }
                if rng.gen::<f64>() <= 0.25 && !connection.enabled {
                    connection.enabled = true;
                    establish_connection = false;
                }
            }
        }
        if establish_connection {
            connections.push(Graph::establish_connection(
                unode.id, vnode.id, rng.gen_range(-2.0..=2.0),
                true, innovations
                )
            );
        }
    }

    fn new_node(
        innovations: &mut speciation::Innovations,
        nodes: &mut Vec<Node>, connections: &mut Vec<Connection>) {
        //
        let mut rng = rand::thread_rng();
        let cxns_len = connections.len();
        let mut rand_cxn = connections[rng.gen_range(0..cxns_len)];
        rand_cxn.enabled = false;
        let new_node = Node{ 
            id: nodes.len(), kind: 0, layer: 0, sum_input: 0, sum_output: 0 };
        nodes.push(new_node);

        // back half
        connections.push(Graph::establish_connection(
            rand_cxn.unode, new_node.id, rand_cxn.weight, 
            true, innovations
            )
        );
        // forward half
        connections.push(Graph::establish_connection(
            new_node.id, rand_cxn.vnode, rng.gen_range(-2.0..=2.0), 
            true, innovations
            )
        );
        //Graph::set_layers();
    }

    
}

