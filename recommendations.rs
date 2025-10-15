use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use crate::models::Produto;

pub fn connect_similar_products(grafo: &mut Graph<Produto, (), petgraph::Undirected>, indices: &[NodeIndex]) {
    for i in 0..indices.len() {
        for j in (i+1)..indices.len() {
            grafo.add_edge(indices[i], indices[j], ());
        }
    }
}

pub fn search_by_name(grafo: &Graph<Produto, (), petgraph::Undirected>, nome_map: &HashMap<String, NodeIndex>, nome: &str) -> Vec<Produto> {
    nome_map.iter()
        .filter(|(k, _)| k.contains(nome))
        .filter_map(|(_, &idx)| grafo.node_weight(idx).cloned())
        .collect()
}

pub fn search_by_category(grafo: &Graph<Produto, (), petgraph::Undirected>, categoria_map: &HashMap<String, Vec<NodeIndex>>, categoria: &str) -> Vec<Produto> {
    categoria_map.get(categoria)
        .map(|indices| indices.iter().filter_map(|&idx| grafo.node_weight(idx).cloned()).collect())
        .unwrap_or_default()
}

pub fn recommend_products(grafo: &Graph<Produto, (), petgraph::Undirected>, idx: NodeIndex) -> Vec<Produto> {
    grafo.neighbors(idx)
        .filter_map(|n| grafo.node_weight(n).cloned())
        .collect()
}
