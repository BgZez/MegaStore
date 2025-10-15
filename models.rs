#[derive(Debug, Clone)]
pub struct Produto {
    pub nome: String,
    pub categoria_p: Categoria,
    pub preco: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Categoria {
    Computadores,
    Smartphones,
    Acessorios,
}

impl Categoria {
    pub fn nome(&self) -> &str {
        match self {
            Categoria::Computadores => "computadores",
            Categoria::Smartphones => "smartphones",
            Categoria::Acessorios => "acessorios",
        }
    }
}
