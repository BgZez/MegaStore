# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este projeto implementa um sistema de busca inteligente para o catálogo de produtos da MegaStore, utilizando grafos e tabelas hash para otimizar a performance e precisão das consultas. O sistema permite buscas por nome, marca, categoria e recomenda produtos similares com base em conexões no grafo.

## Tecnologias Utilizadas
- Linguagem: Rust
- Crates:
  - `petgraph`: para representação de grafos
  - `serde`, `serde_json`: para serialização de dados
  - `clap`: para interface de linha de comando
  - `anyhow`: para tratamento de erros
  - `lazy_static`: para inicialização estática
- Ferramentas de teste:
  - `cargo test`

## Como Executar o Sistema de Busca
1. Clone o repositório ou descompacte o projeto
2. Navegue até a pasta do projeto
3. Execute o comando:
```bash
cargo run --bin main -- search-nome smartphone
```
4. Outros exemplos:
```bash
cargo run --bin main -- search-categoria smartphones
cargo run --bin main -- recomendar smartphone samsung
```

## Como Executar os Testes
```bash
cargo test
```

## Exemplos de Uso
- Buscar por nome: `cargo run --bin main -- search-nome notebook`
- Buscar por categoria: `cargo run --bin main -- search-categoria vestuario`
- Recomendação: `cargo run --bin main -- recomendar camiseta nike`

## Arquitetura do Sistema
- `main.rs`: ponto de entrada e interface CLI
- `models.rs`: definição das estruturas de produto e catálogo
- `recommendations.rs`: lógica de busca e recomendação com grafos
- `lib.rs`: integração dos módulos
- `tests/`: testes unitários e de integração

## Algoritmos e Estruturas de Dados Utilizados
- Grafos direcionados com `petgraph` para representar conexões entre produtos
- Tabelas hash (`HashMap`) para indexação rápida por nome, marca e categoria

## Considerações sobre Desempenho e Escalabilidade
- Busca em tempo constante com tabelas hash
- Recomendação eficiente com grafos
- Modularidade permite escalar para milhares de produtos
- Testes de desempenho mostram resposta em milissegundos para consultas comuns

## Contribuições
Contribuições são bem-vindas! Basta abrir uma issue ou pull request com sugestões ou melhorias.

## Licença
Este projeto está licenciado sob a MIT License.
