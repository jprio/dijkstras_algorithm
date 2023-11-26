use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use petgraph::dot::Config;
use petgraph::graph::NodeIndex;
use petgraph::graph::Graph;
use petgraph::dot::Dot;
use petgraph::algo;
use petgraph::prelude::UnGraph;
use petgraph::visit::EdgeRef;
use petgraph::visit::{ VisitMap, Visitable };

fn main() {
    let mut graph = Graph::new_undirected();
    let start: NodeIndex = graph.add_node("start");
    let a: NodeIndex = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    let e = graph.add_node("e");
    let f = graph.add_node("f");
    graph.add_edge(start, a, 2f32);
    graph.add_edge(start, b, 6f32);
    graph.add_edge(a, c, 5f32);
    graph.add_edge(b, c, 8f32);
    graph.add_edge(c, d, 10f32);
    graph.add_edge(c, e, 15f32);
    graph.add_edge(d, e, 6f32);
    graph.add_edge(e, f, 6f32);
    graph.add_edge(d, f, 2f32);

    println!("{}", Dot::new(&graph));

    // ots_dijkstra(&graph);
    dijkstra(graph, start);
}

fn ots_dijkstra(graph: &Graph<&str, f32>) {
    for start in graph.node_indices() {
        println!("--- {:?} ---", start.index());
        println!(
            "{:?}",
            algo::dijkstra(&graph, start, None, |_| 1)
        );
    }
}

fn dijkstra(graph: UnGraph<&str, f32>, start: NodeIndex) {
    // nodes that have been included in the path
    let mut visited = graph.visit_map();
    // Distances initialized with a big number
    let mut scores = graph
        .node_indices()
        .into_iter()
        .map(|node_index| (node_index, f32::MAX))
        .collect::<HashMap<_, _>>();

    // we can mark the start node as visited, and the score for him is 0
    scores.insert(start, 0f32);
    visited.visit(start);

    let mut visit_next: VecDeque<NodeIndex> = VecDeque::new();
    visit_next.push_back(start);

    while let Some(node) = visit_next.pop_back() {
        println!("exploring from {:#?}", graph.node_weight(node).unwrap());

        // exploration of adjacents nodes
        for edge in graph.edges(node) {
            let target_node = &edge.target();
            println!(
                "target node: {:#?}, distance : {}",
                graph.node_weight(*target_node).unwrap(),
                edge.weight()
            );

            if visited.is_visited(target_node) {
                println!("{:#?} already visited", graph.node_weight(*target_node).unwrap());
                continue;
            }
            let distance = edge.weight();
            if scores.get(target_node).unwrap() > &(distance + scores.get(&node).unwrap()) {
                println!(
                    "updating {:?} score : {:?} (was {:?})",
                    graph.node_weight(*target_node).unwrap(),
                    edge.weight(),
                    scores.get(target_node).unwrap()
                );
                scores.insert(*target_node, distance + scores.get(&node).unwrap());
            }
            visit_next.push_back(*target_node);
        }

        let min = graph
            .edges(node)
            .into_iter()
            .filter(|edge| !visited.is_visited(&edge.target()))
            .fold(None, |min, x| {
                match min {
                    None => Some(x),
                    Some(y) => Some(if x.weight() < y.weight() { x } else { y }),
                }
            });

        match min {
            Some(_) => {
                let target = min.unwrap().target();
                println!(
                    "closer to {:#?} : {:#?}",
                    graph.node_weight(node).unwrap(),
                    graph.node_weight(target).unwrap()
                );

                visited.visit(target);
            }
            None => {
                continue;
            }
        }

        println!("===================================================================");
    }
    for score in scores {
        println!("{:?} : {}", graph.node_weight(score.0).unwrap(), score.1);
    }
    println!("{}", visited);
}
