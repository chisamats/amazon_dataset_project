use std::collections::HashMap;
use std::collections::VecDeque;

type Graph = HashMap<usize, usize>;

pub fn out_degree_adjacency_list(graph: &Graph) -> HashMap<usize, Vec<usize>> {
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();

    for (&node, &neighbor) in graph {
        adj_list.entry(neighbor).or_insert_with(Vec::new).push(node);
    }

    adj_list
}

//bfs six degrees of separation
fn bfs_degrees_of_separation(graph: &HashMap<usize, Vec<usize>>, start: usize) -> HashMap<usize, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        let current_dist = distances[&v];
        if let Some(neighbors) = graph.get(&v) {
            for u in neighbors {
                if !distances.contains_key(u) { // consider all unprocessed neighbors of v
                    let new_dist = current_dist + 1;
                    distances.insert(*u, new_dist);
                    queue.push_back(*u);
                }
            }
        }
    }

    distances
}

pub fn graph_degrees_of_separation(edges: &HashMap<usize, usize>) {
    let graph = out_degree_adjacency_list(&edges);
    let nodes = graph.len();
    let mut count = 0;
    let mut all_distances = 0;
    for n in 0..nodes {
        let distances = bfs_degrees_of_separation(&graph, n);
        all_distances += distances.len();
        for m in 0..nodes {
            if let Some(&distance) = distances.get(&m) {
                if distance > 6 {
                     count += 1;
                }
            }
        }
    }

    println!("{}/{} = {} paths between nodes are greater than 6", count, all_distances, count as f64/all_distances as f64);
}


#[test]
fn test_bfs_degrees_of_separation() {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![1, 4]);
    graph.insert(3, vec![1, 5]);
    graph.insert(4, vec![2]);
    graph.insert(5, vec![3]);

    let distances = bfs_degrees_of_separation(&graph, 1);

    let mut expected_distances = HashMap::new();
    expected_distances.insert(1, 0); 
    expected_distances.insert(2, 1); 
    expected_distances.insert(3, 1); 
    expected_distances.insert(4, 2); 
    expected_distances.insert(5, 2); 

    assert_eq!(distances, expected_distances);
}