use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::{self};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use graphix::GraphRep;

// reads a graph from a file
// and puts it into a graphix graph representation
// format of the file is:
// start_vertex end_vertex weight
// start_vertex end_vertex weight
// start_vertex end_vertex weight
pub fn read<K>(file_path: &str) -> io::Result<GraphRep<K>>
where
    K: FromStr + Copy + PartialOrd,
    K::Err: std::fmt::Debug,
{
    // first pass to get the number of vertices and edges
    let file = File::open(file_path)?;
    let file_content = BufReader::new(file);

    let mut edge_count = 0;
    let mut vertex_count = 0;

    for line in file_content.lines() {
        let line = line?;
        let edge: Vec<_> = line.split_whitespace().collect();

        if edge.len() != 3 {
            continue;
        }

        // gets both verticies from each line
        let vertex_1: usize = edge[0].parse().expect("invalid vertex");
        let vertex_2: usize = edge[1].parse().expect("invalid vertex");

        //checks if vertex_1 or vertex_2 are bigger
        //than the current biggest vertex
        vertex_count = vertex_count.max(vertex_1).max(vertex_2);
        edge_count += 1;
    }

    let mut graph = GraphRep::<K>::new(vertex_count + 1, edge_count);

    //second pass to add edges to the graph
    let file = File::open(file_path)?;
    let file_content = BufReader::new(file);

    for line in file_content.lines() {
        let line = line?;
        let edge: Vec<_> = line.split_whitespace().collect();

        if edge.len() != 3 {
            continue;
        }

        let start_vertex: usize = edge[0].parse().expect("invalid vertex");
        let end_vertex: usize = edge[1].parse().expect("invalid vertex");
        let weight: K = edge[2].parse().expect("invalid edge");

        graph.add_edge(start_vertex, end_vertex, weight);
    }

    graph.finish_v();

    Ok(graph)
}

pub fn write<K>(graph: &GraphRep<K>, file_path: &str) -> io::Result<()>
where
    K: Display + Copy + PartialEq + PartialOrd,
{
    let file = File::create(file_path)?;
    let mut content = BufWriter::new(file);

    // we no longer need Hash+Eq on K – just keep a Vec and do a linear scan
    let mut seen_edges: Vec<(usize, usize, K)> = Vec::new();

    for u in 0..graph.num_vertices() {
        for (v, weight) in graph.edges_from(u) {
            // skip if we've already seen this edge in either direction
            if seen_edges.iter().any(|&(x, y, w)| {
                (x == u && y == v && w == weight) || (x == v && y == u && w == weight)
            }) {
                continue;
            }
            writeln!(content, "{} {} {}", u, v, weight)?;
            seen_edges.push((u, v, weight));
        }
    }
    Ok(())
}
