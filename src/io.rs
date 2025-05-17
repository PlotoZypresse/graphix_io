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
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Collect every valid line into our edge list
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 3 {
            continue;
        }
        let u: usize = parts[0].parse::<usize>().unwrap() - 1;
        let v: usize = parts[1].parse::<usize>().unwrap() - 1;
        let w: K = parts[2].parse().expect("invalid weight");
        edges.push((u, v, w));
    }

    // One‐shot CSR build
    Ok(GraphRep::from_list(edges))
}

pub fn write<K>(graph: &GraphRep<K>, file_path: &str) -> io::Result<()>
where
    K: Display + Copy + PartialEq + PartialOrd,
{
    let file = File::create(file_path)?;
    let mut out = BufWriter::new(file);

    // Simply re-emit each original undirected edge once
    for &(u, v, w) in &graph.id {
        writeln!(out, "{} {} {}", u, v, w)?;
    }

    Ok(())
}
