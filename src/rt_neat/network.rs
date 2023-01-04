struct Node {
    id:         i32,
    kind:       i32,
    layer:      i32,
    sum_input:  i32,
    sum_output: i32,
}

struct Connection {
    innovation: i32,
    u_node:     i32,
    v_node:     i32,
    weight:     i32,
    enabled:    bool,
}

struct Network {
    nodes:          Vec<Node>,
    connections:    Vec<Connection>,
}

