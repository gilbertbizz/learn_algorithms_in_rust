use std::{collections::{HashMap, HashSet}, fmt::Display, mem::transmute_copy};

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool;
    fn add_edge(&mut self, edge: (&str, &str, i32));
    fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, Node>;
    fn contains(&self, node: &str) -> bool;
    fn nodes(&self) -> HashSet<&String>;
    fn edges(&self) -> Vec<(&String, &String, i32)>;
}


 #[derive(Debug, Clone)]
 pub struct Node;

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

#[derive(Debug)]
pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new()
        }
    }

    fn adjacency_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table_mut()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, Node> {
        match self.adjacency_table().get(node) {
            None => Err(Node),
            Some(i) => Ok(i),
        }
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();

        for (from_node, from_node_neighbors) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbors {
                edges.push((from_node, to_node, *weight));
        }
      }
     edges
    }

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mut()
                    .insert((*node).to_string(), Vec::new());
                true 
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}



impl Graph for UndirectedGraph {
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new()
        }
    }


    fn adjacency_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }

    fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, Node> {
        match self.adjacency_table().get(node) {
            None => Err(Node),
            Some(i) => Ok(i),
        }
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();

        for (from_node, from_node_neighbors) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbors {
                edges.push((from_node, to_node, *weight));
        }
      }
     edges
    }

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mut()
                    .insert((*node).to_string(), Vec::new());
                true 
            }
            _ => false,
        }
    }
}