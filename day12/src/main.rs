use anyhow::{Context, Result};
use petgraph::{algo::dijkstra, graph::Graph, graph::NodeIndex, prelude::*};

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1(&data)?, 31);
    assert_eq!(part2(&data)?, 29);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Node {
    x: i32,
    y: i32,
    ch: char,
}

fn part1(data: &str) -> Result<i32> {
    let MetaGraph {
        graph,
        graph_nodes,
        start,
        end,
        ..
    } = new_graph(data)?;

    let path = dijkstra(&graph, graph_nodes[start], Some(graph_nodes[end]), |_| 1);
    let steps = path.get(&graph_nodes[end]).context("no entry")?;

    Ok(*steps)
}

fn part2(data: &str) -> Result<i32> {
    let MetaGraph {
        graph,
        nodes,
        graph_nodes,
        end,
        ..
    } = new_graph(data)?;

    let a_indices: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter_map(|(idx, node)| {
            if node.ch == 'a' {
                return Some(idx);
            }
            None
        })
        .collect();

    let steps = a_indices
        .iter()
        .filter_map(|idx| {
            let path = dijkstra(&graph, graph_nodes[*idx], Some(graph_nodes[end]), |_| 1);
            let steps = path.get(&graph_nodes[end])?;
            Some(*steps)
        })
        .collect::<Vec<i32>>();
    let min = steps.iter().min().context("no minimum")?;

    Ok(*min)
}

struct MetaGraph {
    graph: Graph<Node, (), Directed>,
    nodes: Vec<Node>,
    graph_nodes: Vec<NodeIndex<u32>>,
    start: usize,
    end: usize,
}

fn new_graph(data: &str) -> Result<MetaGraph> {
    let mut start = 0;
    let mut end = 0;
    let mut nodes = Vec::new();
    let mut graph_nodes = Vec::new();

    let mut graph: Graph<Node, (), Directed> = Graph::new();
    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let ch = match ch {
                'S' => {
                    start = nodes.len();
                    'a'
                }
                'E' => {
                    end = nodes.len();
                    'z'
                }
                v => v,
            };
            let node = Node {
                x: x as i32,
                y: y as i32,
                ch,
                ..Default::default()
            };
            nodes.push(node.clone());
            graph_nodes.push(graph.add_node(node));
        }
    }

    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];

            if ((a.y == b.y) && (a.x == b.x - 1 || a.x == b.x + 1))
                || ((a.x == b.x) && (a.y == b.y - 1 || a.y == b.y + 1))
            {
                let diff = b.ch as i32 - a.ch as i32;
                if diff <= 1 {
                    graph.add_edge(graph_nodes[i], graph_nodes[j], ());
                }
            }
        }
    }

    Ok(MetaGraph {
        graph,
        nodes,
        graph_nodes,
        start,
        end,
    })
}
