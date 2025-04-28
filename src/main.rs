use graphix_io::io::{read, write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = read::<f32>("test_files/test_graph_float.txt")?;
    write(&graph, "test_files/output_graph.txt")?;
    Ok(())
}
