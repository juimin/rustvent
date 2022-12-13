use advent22::get_input_contents;
use petgraph::{
    graph::{Graph, NodeIndex},
    visit::IntoNeighbors,
};

fn build_fs_graph(contents: &String) -> Graph<&str, i32> {
    let mut graph = Graph::<&str, i32>::new();
    let mut current_node = graph.add_node("");
    let slash_node = graph.add_node("/");
    graph.add_edge(current_node, slash_node, 0);

    for line in contents.trim().split("\n") {
        let mut tokens = line.split_whitespace();
        let first_token = tokens.next().expect("Line should not be empty");
        if first_token == "$" {
            let command = tokens.next().expect("$ Should have command after it");
            if command == "cd" {
                let target_dir = tokens.next().expect("There should be a directory after cd");
                for n in graph.neighbors(current_node) {
                    let node_weight = graph.node_weight(n).expect("node exists");
                    let edge = graph
                        .find_edge(current_node, n)
                        .expect("These are all neightbors");
                    let edge_weight = graph.edge_weight(edge).expect("edge exists");
                    if (target_dir == ".." && *edge_weight == -1) || (*node_weight == target_dir) {
                        current_node = n;
                        break;
                    }
                }
            }
        } else if first_token == "dir" {
            // This should be a new directory
            let child_dir = tokens.next().expect("There should be a directory name");
            let child_node = graph.add_node(child_dir);
            graph.add_edge(current_node, child_node, 0);
            graph.add_edge(child_node, current_node, -1);
        } else {
            // This should be a file and first token is the file size
            let file_name = tokens.next().expect("There should be a file name");
            let file_node = graph.add_node(file_name);
            graph.add_edge(
                current_node,
                file_node,
                first_token.parse::<i32>().expect("This should be an int"),
            );
        }
    }

    return graph;
}

fn calculate_size(graph: &Graph<&str, i32>, current_node: NodeIndex) -> i32 {
    let mut total = 0;

    for neighbor in graph.neighbors(current_node) {
        let edge = graph
            .find_edge(current_node, neighbor)
            .expect("we're neighbors so this should exist");
        let edge_weight_ptr = graph.edge_weight(edge).expect("edge should have weight");
        let edge_weight = *edge_weight_ptr;
        if edge_weight > 0 {
            // Means nieghbor is a file and in this dir
            total += edge_weight;
        } else if edge_weight == 0 {
            // Means neighbor is a subdir
            total += calculate_size(graph, neighbor);
        }
        // We don't need to go backwards with parent refs since those have a negative weight
    }

    return total;
}

fn calculate_answer_1(graph: &Graph<&str, i32>, limit: i32) -> i32 {
    let mut total = 0;

    for node in graph.node_indices() {
        let node_label = graph.node_weight(node).expect("node exists");
        let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();
        if neighbors.len() > 0 {
            let size = calculate_size(graph, node);
            if size <= limit {
                total += size;
            }
            println!("Size of dir {}: {}", node_label, size);
        }
    }

    return total;
}

fn main() {
    let file_contents: String = get_input_contents();

    let graph = build_fs_graph(&file_contents);
    let answer_1 = calculate_answer_1(&graph, 100000);

    println!("Sum of total sizes of dirs: {}", answer_1);
    // let limit = 100000;
}
