use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut distances: HashMap<Node, Cost> = HashMap::new();
    let mut previous: HashMap<Node, Node> = HashMap::new();
    let mut frontier: BinaryHeap<(Reverse<Cost>, Node)> = BinaryHeap::new();

    distances.insert(start, 0);
    frontier.push((Reverse(0), start));

    while let Some((Reverse(current_cost), node)) = frontier.pop() {
        if node == goal {
            let mut path = vec![goal];
            let mut current = goal;

            while let Some(&parent) = previous.get(&current) {
                path.push(parent);
                current = parent;
            }

            path.reverse();
            return Some((path, current_cost));
        }

        if current_cost > *distances.get(&node).unwrap_or(&usize::MAX) {
            continue;
        }

        for &(neighbor, edge_cost) in g.edges.get(&node).into_iter().flatten() {
            let next_cost = current_cost + edge_cost;
            let best_known = distances.get(&neighbor).copied().unwrap_or(usize::MAX);

            if next_cost < best_known {
                distances.insert(neighbor, next_cost);
                previous.insert(neighbor, node);
                frontier.push((Reverse(next_cost), neighbor));
            }
        }
    }

    None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
