use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::config;
use crate::rt_neat::species;
use crate::rt_neat::network;

pub fn mutate(
    cfg: &config::Mutation, innovations: &mut species::Innovations,
    network: &mut network::Network) {
    //
    mutate_weights(&cfg.weight, network);
    random_connection(&cfg.random_connection, innovations, network);
    //self.insert_node(innovations);
}

fn mutate_weights(
    cfg: &config::Weight, network: &mut network::Network) {
    //
    let mut rng = rand::thread_rng();
    for cxn in &mut network.connections {
        if rng.gen::<f64>() > cfg.chance { continue }
        if rng.gen::<f64>() > cfg.random {
            cxn.weight = rng.gen_range(-2.0..=2.0);
            continue
        }
        // todo: ensure new cxn.weight does not exceed min/max bounds
        if rng.gen::<f64>() > cfg.add_else_sub {
            cxn.weight *= 1.0 - cfg.sub_factor;
            continue
        }
        if (1.0 + cfg.add_factor) * cxn.weight > 2.0 { continue }
        cxn.weight *= 1.0 + cfg.add_factor;
    }
}

fn random_connection(
    random_connection: &f64, innovations: &mut species::Innovations,
    network: &mut network::Network) {
    //
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() > *random_connection { return }

    let mut nodes_by_layer = HashMap::new();
    for n in &network.nodes {
        nodes_by_layer.entry(&n.layer).or_insert_with(Vec::new).push(&n.id)
    }
    let max_layer = **nodes_by_layer.keys().max()
        .expect("hashmap should not be empty");
    let unodel = rng.gen_range(1..max_layer);
    let vnodel = rng.gen_range(unodel+1..=max_layer); 
    let unode = nodes_by_layer[&unodel].choose(&mut rng).unwrap();
    let vnode = nodes_by_layer[&vnodel].choose(&mut rng).unwrap();

    for cxn in &mut network.connections {
        if !(cxn.unode == **unode && cxn.vnode == **vnode) { continue }    
        if rng.gen::<f64>() > 0.25 { return }
        match cxn.enabled {
            true => cxn.enabled = false,
            false => cxn.enabled = true,
        }
        return
    }
    network.establish_connection(
        **unode, **vnode, None,
        true, innovations
    );
}

fn insert_node(
    cfg: &config::Mutation, innovations: &mut species::Innovations, 
    network: &mut network::Network) {
    //
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() > cfg.insert_node { return }

    let mut rand_cxn = network.connections[rng
        .gen_range(0..network.connections.len())];
    rand_cxn.enabled = false;
    let new_node = network::Node{ 
        id: network.nodes.len(), kind: 0, layer: 0, 
        sum_input: 0, sum_output: 0 };
    network.insert_node(&new_node);
    network.establish_connection( // back half
        rand_cxn.unode, new_node.id, Some(rand_cxn.weight), 
        true, innovations
    );
    network.establish_connection( // forward half
        new_node.id, rand_cxn.vnode, None, 
        true, innovations
    );
    //set_layers(&network);
}

fn set_layers(
    network: &mut network::Network) {
    //
    let g = generate_topology(&network);
    //let mut seen: Vec<i32> = Vec::new();
    //let mut path: Vec<i32> = Vec::new();
    //println!("{:?}", g);
    
    for node in &mut network.nodes {
        if node.kind == 1 { continue }
        let all_paths = dfs(&g, node.id, &Vec::new(), Vec::new());
        let all_paths_max = all_paths.iter().max()
            .expect("slice should not be empty");
        node.layer = all_paths_max + 1;
    }
}

fn generate_topology(
    network: &network::Network) -> HashMap<usize, Vec<usize>> {
    //
    let mut g = HashMap::new();
    for cxn in &network.connections {
        if !cxn.enabled { continue }
        g.entry(cxn.vnode).or_insert_with(Vec::new).push(cxn.unode);
        //g.entry(cxn.unode).or_insert_with(Vec::new).push(cxn.vnode);     
    }
    g
}

fn dfs(
    g: &HashMap<usize, Vec<usize>>, v: usize, 
    seen: &Vec<usize>, mut path: Vec<usize>) -> Vec<usize> {
    //
    if path.is_empty() { path.push(v) }
    //seen.unwrap_or(Vec::new()).push(v);
    let mut paths = Vec::new();
    for t in &g[&v] {
        if seen.contains(t) { continue }
        let mut tpath = path.to_vec();
        tpath.push(*t);
        let to_push = dfs(g, *t, seen, tpath);
        for e in &to_push {
            paths.push(*e);
            //paths.push(dfs(g, *t, seen, tpath));
        }
    }
    paths
} 