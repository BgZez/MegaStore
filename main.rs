use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::env;
use megastore_graphs::models::{Produto, Categoria};
use megastore_graphs::recommendations::{connect_similar_products, search_by_name, search_by_category, recommend_products};

fn criar_grafo_e_mapas() -> (Graph<Produto, (), Undirected>, HashMap<String, NodeIndex>, HashMap<String, Vec<NodeIndex>>) {
    let mut grafo = Graph::<Produto, (), Undirected>::new_undirected();
    let mut nome_map = HashMap::new();
    let mut categoria_map: HashMap<String, Vec<NodeIndex>> = HashMap::new();

    let produtos = vec![
        Produto { nome: "Notebook Dell".into(), categoria_p: Categoria::Computadores, preco: 3500.0 },
        Produto { nome: "Notebook HP".into(), categoria_p: Categoria::Computadores, preco: 4400.0 },
        Produto { nome: "Notebook Acer".into(), categoria_p: Categoria::Computadores, preco: 5500.0 },
        Produto { nome: "Smartphone Xiaomi".into(), categoria_p: Categoria::Smartphones, preco: 2700.0 },
        Produto { nome: "Fone Havit".into(), categoria_p: Categoria::Acessorios, preco: 150.0 },
        Produto { nome: "Smartphone Samsung".into(), categoria_p: Categoria::Smartphones, preco: 2500.0 },
        Produto { nome: "Teclado Redragon".into(), categoria_p: Categoria::Acessorios, preco: 200.0 },
        Produto { nome: "Mouse Logitech".into(), categoria_p: Categoria::Acessorios, preco: 150.0 },
        Produto { nome: "Smartphone Apple".into(), categoria_p: Categoria::Smartphones, preco: 5200.0 },
    ];

    let mut indices = vec![];
    for produto in produtos {
        let idx = grafo.add_node(produto.clone());
        nome_map.insert(produto.nome.to_lowercase(), idx);
        categoria_map.entry(produto.categoria_p.nome().to_string()).or_default().push(idx);
        indices.push(idx);
    }

    connect_similar_products(&mut grafo, &indices);
    (grafo, nome_map, categoria_map)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Uso: cargo run --bin main -- <comando> <argumento>");
        eprintln!("Comandos disponíveis: search-nome, search-categoria, recomendar");
        return;
    }

    let comando = &args[1];
    let argumento = args[2..].join(" " ).to_lowercase();

    let (grafo, nome_map, categoria_map) = criar_grafo_e_mapas();

    match comando.as_str() {
        "search-nome" => {
            let produtos = search_by_name(&grafo, &nome_map, &argumento);
            if produtos.is_empty() {
                println!("Nenhum produto encontrado com o nome '{}'.", argumento);
            } else {
                for p in produtos {
                    println!("Nome: {}, Categoria: {}, Preço: R${}", p.nome, p.categoria_p.nome(), p.preco);
                }
            }
        },
        "search-categoria" => {
            let produtos = search_by_category(&grafo, &categoria_map, &argumento);
            if produtos.is_empty() {
                println!("Nenhum produto encontrado na categoria '{}'.", argumento);
            } else {
                for p in produtos {
                    println!("Nome: {}, Categoria: {}, Preço: R${}", p.nome, p.categoria_p.nome(), p.preco);
                }
            }
        },
        "recomendar" => {
            if let Some(&idx) = nome_map.get(&argumento) {
                let recomendados = recommend_products(&grafo, idx);
                println!("Produtos recomendados para '{}':", argumento);
                for p in recomendados {
                    println!("Nome: {}, Categoria: {}, Preço: R${}", p.nome, p.categoria_p.nome(), p.preco);
                }
            } else {
                println!("Produto '{}' não encontrado para recomendação.", argumento);
            }
        },
        _ => {
            eprintln!("Comando desconhecido.");
        }
    }
}
