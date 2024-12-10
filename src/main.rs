use std::error::Error;
use std::fs::File;
use std::io::BufRead;

mod degree_dist;
mod six_degrees;

fn main() {
    use degree_dist;
    use six_degrees;

    let graph = read_file("amazon0312.txt").unwrap();

    let in_degree_graph = degree_dist::in_degree(&graph);

    let degree_dist = degree_dist::degree_distribution(&in_degree_graph);

    if let Err(e) = degree_dist::build_chart(&degree_dist) {
        eprintln!("Error building chart: {}", e);
    }

    let top_5 = degree_dist::top_five(&in_degree_graph);
    println!("Most Co-purchased Products(product number, connections): {:?}", top_5); 

    let only_1s = degree_dist::bottom(&in_degree_graph);
    println!("Number of Products only Co-purchased with 1 other item: {}", only_1s); 


    six_degrees::graph_degrees_of_separation(&graph);
}

fn read_file(filename: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let data = File::open(filename).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(data);

    let mut edges: Vec<(usize, usize)> = Vec::new();

    for line in buf_reader.lines() {
        let line_str = line?;
        if !line_str.contains("#") {
            let v: Vec<usize> = line_str.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let from_node = v[0];
            let to_node = v[1];
            edges.push((from_node, to_node));        
        }
    }
    Ok(edges)
}
