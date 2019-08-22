#[macro_use]
extern crate maplit;

pub mod graph {
    use std::collections::HashMap;
    pub type Attribute<'a> = (&'a str, &'a str);

    pub mod graph_items {
        pub mod edge {

            use super::super::Attribute;
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge<'a> {
                pub attrs: HashMap<String, String>,
                from: &'a str,
                to: &'a str,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        from,
                        to,
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[Attribute]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    self
                }
            }
        }

        pub mod node {
            use super::super::Attribute;
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node<'a> {
                pub name: String,
                pub attrs: HashMap<String, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &str) -> Self {
                    Node {
                        attrs: hashmap! {},
                        name: String::from(name),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[Attribute<'a>]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| (key.to_string(), *value))
                        .collect();
                    self
                }

                pub fn get_attr(self, key: &str) -> Option<&'a str> {
                    self.attrs.get(&String::from(key)).map(|value| value.clone())
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

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

        pub fn get_node(self, name: &str) -> Option<Node<'a>> {
            self.nodes
                .into_iter()
                .find(|node| node.name == String::from(name))
        }
    }
}
