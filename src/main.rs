use graphix_io::io::{read, write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = read::<i32>("test_files/test_graph.txt")?;
    write(&graph, "test_files/output_graph.txt")?;
    Ok(())
}
