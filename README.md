# graphix_io

[![Crates.io](https://img.shields.io/crates/v/graphix_io.svg)](https://crates.io/crates/graphix_io)  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A minimal Rust I/O helper for loading and saving undirected graphs in plain-text edge-list format, built on top of [`graphix`](https://crates.io/crates/graphix).

---

## Features

- **Read**
  - `read<K>(path: &str) -> io::Result<GraphRep<K>>`
  - Parses every valid line of `u v w` into `Vec<(usize,usize,K)>`, then calls `GraphRep::from_list`.
  - **Trait bounds:** `K: FromStr + Copy`
  - Skips blank or malformed lines silently.

- **Write**
  - `write<K>(graph: &GraphRep<K>, path: &str) -> io::Result<()>`
  - Emits each original undirected edge exactly once by iterating `graph.id`.
  - **Trait bounds:** `K: Display + Copy`

- Zero panics on empty files.
- No hash tables or manual “seen” tracking—just buffered I/O and CSR’s `id` array.

---

## Installation

```toml
[dependencies]
graphix_io = "0.3"
graphix    = "0.4"
````

Or via Cargo:

```bash
cargo add graphix_io@0.3 graphix@0.4
```

---

## Usage

### Input format

Plain-text file where each line is:

```
<u> <v> <w>
```

* `<u>`, `<v>` are `usize` vertex IDs
* `<w>` is your weight type `K::from_str` parses

Blank or malformed lines are skipped.

### Example

```rust
use graphix::GraphRep;
use graphix_io::io::{read, write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Read into CSR graph
    let graph: GraphRep<f64> = read("input.txt")?;
    println!("Loaded {} vertices, {} edges",
             graph.num_vertices(),
             graph.num_edges());

    // … run algorithms …

    // 2) Write back original edge list
    write(&graph, "output.txt")?;
    Ok(())
}
```

Run with:

```bash
cargo run
```

---

## API

```rust
pub fn read<K>(file_path: &str) -> io::Result<GraphRep<K>>
where
    K: FromStr + Copy,
    K::Err: Debug;
```

```rust
pub fn write<K>(graph: &GraphRep<K>, file_path: &str) -> io::Result<()>
where
    K: Display + Copy;
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
