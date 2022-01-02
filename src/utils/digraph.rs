use crate::utils::dna::Dna;
use crate::utils::fasta::Fasta;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Node<T: Eq> {
    name: String,
    value: T,
}

impl<T: Eq> Node<T> {
    fn new(name: String, value: T) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

pub struct Digraph<T: Eq> {
    edges: BTreeMap<Node<T>, BTreeSet<Node<T>>>,
}

impl<T: Eq + Ord> Digraph<T> {
    fn new() -> Self {
        Self {
            edges: BTreeMap::new(),
        }
    }

    fn insert_edge(&mut self, from: Node<T>, to: Node<T>) {
        self.edges
            .entry(from)
            .or_insert_with(BTreeSet::new)
            .insert(to);
    }

    pub fn adjacency_list(&self) -> Vec<(&Node<T>, &Node<T>)> {
        let mut adj_list = Vec::new();
        for (node1, nodelist) in &self.edges {
            for node2 in nodelist {
                adj_list.push((node1, node2))
            }
        }
        adj_list
    }
}

impl Digraph<&Dna> {
    pub fn overlap_graph_from_fasta_dna(fasta: &Fasta<Dna>, k: usize) -> Digraph<&Dna> {
        let mut graph = Digraph::new();
        for node1 in fasta.iter() {
            for node2 in fasta.iter() {
                if node1 != node2 && node1.1.suffix(k) == node2.1.prefix(k) {
                    graph.insert_edge(
                        Node::new(node1.0.to_string(), &node1.1),
                        Node::new(node2.0.to_string(), &node2.1),
                    )
                }
            }
        }
        graph
    }
}
