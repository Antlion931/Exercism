pub mod graph {
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub body: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(body: impl AsRef<str>) -> Self {
                    Self {
                        body: body.as_ref().to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, key: impl AsRef<str>) -> Option<&str> {
                    self.attrs.get(key.as_ref()).map(String::as_str)
                }

                pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
                    for (a, b) in attributes {
                        self.attrs.insert(a.to_string(), b.to_string());
                    }

                    self
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: impl AsRef<str>, to: impl AsRef<str>) -> Self {
                    Self {
                        from: from.as_ref().to_string(),
                        to: to.as_ref().to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, key: impl AsRef<str>) -> Option<&str> {
                    self.attrs.get(key.as_ref()).map(String::as_str)
                }

                pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
                    for (a, b) in attributes {
                        self.attrs.insert(a.to_string(), b.to_string());
                    }

                    self
                }
            }
        }
    }
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                attrs: HashMap::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
            }
        }

        pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
            for (a, b) in attributes {
                self.attrs.insert(a.to_string(), b.to_string());
            }

            self
        }

        pub fn attr(&self, key: impl AsRef<str>) -> Option<&str> {
            self.attrs.get(key.as_ref()).map(String::as_str)
        }

        pub fn node(&self, key: impl AsRef<str>) -> Option<&Node> {
            self.nodes.iter().find(|n| n.body == key.as_ref())
        }

        pub fn with_nodes(mut self, nodes: impl AsRef<[Node]>) -> Self {
            for n in nodes.as_ref().iter() {
                self.nodes.push(n.clone());
            }

            self
        }

        pub fn with_edges(mut self, edges: impl AsRef<[Edge]>) -> Self {
            for e in edges.as_ref().iter() {
                self.edges.push(e.clone());
            }

            self
        }
    }
}
