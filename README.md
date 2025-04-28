# graphix_io

[![Crates.io](https://img.shields.io/crates/v/graphix_io.svg)](https://crates.io/crates/graphix_io)  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust library for reading and writing graphs from and to text files, based on the [`graphix`](https://crates.io/crates/graphix) graph representation.
It provides a simple and efficient way to load and save graphs for use in graph algorithms or visualization tools.

---

## Features

- **Read** graphs from text files:
  - File format: `start_vertex end_vertex weight`
  - Each line represents one edge (invalid or malformed lines are skipped).
- **Write** graphs to text files:
  - Automatically ensures undirected edges are written only once
  - Uses a simple in-memory Vec-based scan; no `Hash` or `Eq` bounds required on the weight type
- **Designed to work with** [`graphix::GraphRep<K>`](https://crates.io/crates/graphix).
- **Minimal dependencies**, fast and lightweight.

## Installation

Add `graphix_io` to your `Cargo.toml` dependencies:

```toml
[dependencies]
graphix_io = "0.1"
graphix = "0.2"
```

Or with `cargo`:

```bash
cargo add graphix_io
```

## Quick Start

```rust
use graphix_io::io::{read, write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a graph from a file
    let graph = read::<i32>("test_graph.txt")?;

    // Write the graph to another file
    write(&graph, "output_graph.txt")?;

    Ok(())
}
```

Where `test_graph.txt` contains lines like:

```
0 1 10
1 2 20
2 3 30
3 0 40
```

Each line defines an undirected edge with a weight.

## API

### `read<K>(file_path: &str) -> io::Result<GraphRep<K>>`

Reads a graph from a text file into a `GraphRep<K>`.

- **Trait bounds:** `K: FromStr + Copy + PartialOrd`
- Skips lines that do not have exactly three whitespace-separated tokens.

### `write<K>(graph: &GraphRep<K>, file_path: &str) -> io::Result<()>`

Writes a graph to a text file, emitting each undirected edge exactly once.

- **Trait bounds:** `K: Display + Copy + PartialEq + PartialOrd`
- Does not require `Hash` or `Eq` on `K`; uses a linear scan of a `Vec` to track seen edges.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
