use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;
use plotters::prelude::*;

mod six_degrees;

type Graph = HashMap<usize, usize>;

fn main() {
    use six_degrees;

    let in_degree_graph = read_file("amazon0312.txt").unwrap();

    let degree_dist = degree_distribution(&in_degree_graph);

    if let Err(e) = build_chart(&degree_dist) {
        eprintln!("Error building chart: {}", e);
    }

    let top_5 = top_five(&in_degree_graph);
    println!("Most Co-purchased Products(product number, connections): {:?}", top_5); 

    let only_1s = bottom(&in_degree_graph);
    println!("Number of Products only Co-purchased with 1 other item: {}", only_1s); 


    six_degrees::graph_degrees_of_separation(&in_degree_graph);
}

fn read_file(filename: &str) -> Result<Graph, Box<dyn Error>> {
    let data = File::open(filename).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(data);

    let mut graph: Graph = HashMap::new();

    for line in buf_reader.lines() {
        let line_str = line?;
        if !line_str.contains("#") {
            let v: Vec<usize> = line_str.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let to_node = v[1];
            *graph.entry(to_node).or_insert(0) += 1;
        }
    }
    Ok(graph)
}

fn degree_distribution(graph: &Graph) -> Graph {
    let mut degree_dist_graph: Graph = HashMap::new();
    
    for degree in graph.values() {
        *degree_dist_graph.entry(*degree).or_insert(0) += 1;
    }

    degree_dist_graph
}

fn top_five(graph: &Graph) -> Vec<(usize, usize)> {
    let mut vec: Vec<(usize, usize)> = graph.iter().map(|(&key, &value)| (key, value)).collect();
    
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    let top_5 = vec.into_iter().take(5).collect::<Vec<(usize, usize)>>();

    top_5
}

fn bottom(graph: &Graph) -> u32 {
    let mut vec: Vec<(usize, usize)> = graph.iter().map(|(&key, &value)| (key, value)).collect();
    
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    let mut count = 0;
    for i in vec {
        if i.1 == 1 {
            count +=1;
        }
    }

    count
}

fn build_chart(graph: &Graph) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("degree_distribution_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_degree = *graph.keys().min().unwrap() as f64;
    let max_degree = *graph.keys().max().unwrap() as f64;
    let min_freq = *graph.values().min().unwrap() as f64;
    let max_freq = *graph.values().max().unwrap() as f64;

    let x_min = min_degree.ln();  
    let x_max = max_degree.ln();  
    let y_min = min_freq.ln();    
    let y_max = max_freq.ln(); 

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Distribution Plot on a Log Scale", ("Arial", 20).into_font())
        .x_label_area_size(80)
        .y_label_area_size(80)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    
    chart.configure_mesh()
        .x_desc("Degree (Log Scale)")
        .y_desc("Frequency (Log Scale)")
        .draw()?;

    let data_points: Vec<(f64, f64)> = graph
        .iter()
        .map(|(&degree, &count)| (degree as f64, count as f64))
        .collect();

    chart.draw_series(data_points.iter().map(|&(x, y)| {
            Circle::new((x.ln(), y.ln()), 3, ShapeStyle {
                color: BLACK.to_rgba(),
                filled: true,
                stroke_width: 1,
            })
        }))?;
    
    Ok(())
}


#[test]
fn test_top_five() {
    let mut graph = Graph::new();
    graph.insert(1, 10); 
    graph.insert(2, 30); 
    graph.insert(3, 20); 
    graph.insert(4, 40); 
    graph.insert(5, 15); 
    graph.insert(6, 25); 

    let expected_top_5 = vec![
        (4, 40), 
        (2, 30),
        (6, 25),
        (3, 20), 
        (5, 15), 
    ];

    let top_5 = top_five(&graph);

    assert_eq!(top_5, expected_top_5);
}

#[test]
fn test_bottom() {
    let mut graph = Graph::new();
    graph.insert(1, 10); 
    graph.insert(2, 1);    
    graph.insert(4, 1);  
    graph.insert(5, 5);  
    graph.insert(6, 1);  
    let expected_bottom = 3;

    let result = bottom(&graph);
    assert_eq!(result, expected_bottom);
}