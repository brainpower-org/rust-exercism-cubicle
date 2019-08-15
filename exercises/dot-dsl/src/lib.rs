#[macro_use]
extern crate maplit;

pub mod graph {
    pub type Attribute<'a> = (&'a str, &'a str);

    pub mod graph_items {
        pub mod edge {
            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Edge { from, to }
                }
            }
        }

        pub mod node {
            use super::super::Attribute;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node<'a> {
                pub attrs: Vec<Attribute<'a>>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &str) -> Self {
                    // TODO store name
                    Node {
                        // TODO maybe store as hashmap
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs = attrs.to_vec();
                    self
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap! {},
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node<'a>>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge<'a>>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[Attribute<'a>]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            self
        }
    }
}
