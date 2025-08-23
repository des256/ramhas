use {
    crate::*,
    anyhow::Result,
    graphviz_rust::{
        cmd::{CommandArg, Format},
        dot_structures::{
            Attribute, Edge, EdgeTy, Graph, Id as DotId, Node, NodeId, Port, Stmt, Vertex,
        },
        exec,
        printer::PrinterContext,
    },
    std::{cell::RefCell, collections::HashMap, fs::File, io::Write, path::Path, rc::Rc},
};

pub struct Visualizer {
    next_id: usize,
    nodes: HashMap<u64, Node>,
    edges: Vec<Edge>,
}

pub fn add_attr(attributes: &mut Vec<Attribute>, name: &str, value: &str) {
    attributes.push(Attribute(
        DotId::Plain(name.to_string()),
        DotId::Plain(value.to_string()),
    ));
}

impl Visualizer {
    fn new() -> Self {
        Self {
            next_id: 0,
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    fn generate_id(&mut self) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut id = self.next_id;
        self.next_id += 1;
        let mut result = String::new();
        if id == 0 {
            return "a".to_string();
        }
        while id > 0 {
            let n = id % alphabet.len();
            let c = alphabet.chars().nth(n).unwrap();
            result.insert(0, c);
            id = (id - n) / alphabet.len();
        }
        result
    }

    pub fn add_n2n(&mut self, from: &str, to: &str, red: bool) {
        self.edges.push(Edge {
            ty: EdgeTy::Pair(
                Vertex::N(NodeId(DotId::Plain(from.to_string()), None)),
                Vertex::N(NodeId(DotId::Plain(to.to_string()), None)),
            ),
            attributes: if red {
                vec![Attribute(
                    DotId::Plain("color".to_string()),
                    DotId::Plain("red".to_string()),
                )]
            } else {
                Vec::new()
            },
        });
    }

    pub fn add_n2p(&mut self, from: &str, to: &str, port: &str, red: bool) {
        self.edges.push(Edge {
            ty: EdgeTy::Pair(
                Vertex::N(NodeId(DotId::Plain(from.to_string()), None)),
                Vertex::N(NodeId(
                    DotId::Plain(to.to_string()),
                    Some(Port(Some(DotId::Plain(port.to_string())), None)),
                )),
            ),
            attributes: if red {
                vec![Attribute(
                    DotId::Plain("color".to_string()),
                    DotId::Plain("red".to_string()),
                )]
            } else {
                Vec::new()
            },
        });
    }

    pub fn add_p2n(&mut self, from: &str, port: &str, to: &str, red: bool) {
        self.edges.push(Edge {
            ty: EdgeTy::Pair(
                Vertex::N(NodeId(
                    DotId::Plain(from.to_string()),
                    Some(Port(Some(DotId::Plain(port.to_string())), None)),
                )),
                Vertex::N(NodeId(DotId::Plain(to.to_string()), None)),
            ),
            attributes: if red {
                vec![Attribute(
                    DotId::Plain("color".to_string()),
                    DotId::Plain("red".to_string()),
                )]
            } else {
                Vec::new()
            },
        });
    }

    pub fn add_bindings(
        &mut self,
        _gen_id: &str,
        bindings: &Rc<RefCell<Vec<HashMap<String, Rc<Expr>>>>>,
    ) -> String {
        let index = (&*bindings as *const _) as u64;
        if self.nodes.contains_key(&index) {
            return self.nodes[&index].id.0.to_string();
        }
        let scopes_id = self.generate_id();
        let mut node = Node {
            id: NodeId(DotId::Plain(scopes_id.clone()), None),
            attributes: Vec::new(),
        };
        add_attr(&mut node.attributes, "shape", "record");
        let mut label = "\"{Bindings".to_string();
        for (i, bindings) in bindings.borrow().iter().enumerate() {
            label.push_str(&format!("|{{<scope{}>{}|{{", i, i));
            let mut first = true;
            for (name, expr) in bindings.iter() {
                if first {
                    first = false;
                } else {
                    label.push_str("|");
                }
                let expr_id = self.add_expr(expr);
                label.push_str(&format!("<binding{}_{}>{}", i, name, name));
                self.add_p2n(
                    &scopes_id,
                    &format!("binding{}_{}", i, name),
                    &expr_id,
                    false,
                );
            }
            label.push_str("}}");
        }
        label.push_str("}\"");
        add_attr(&mut node.attributes, "label", &label);
        self.nodes.insert(index, node);
        scopes_id
    }

    pub fn add_ctrl(&mut self, arena: &Arena<Ctrl>, ctrl: &Id<Ctrl>) -> String {
        let index = ctrl as *const _ as u64;
        if self.nodes.contains_key(&index) {
            return self.nodes[&index].id.0.to_string();
        }
        let gen_id = self.generate_id();
        let mut node = Node {
            id: NodeId(DotId::Plain(gen_id.clone()), None),
            attributes: Vec::new(),
        };
        add_attr(&mut node.attributes, "shape", "record");
        add_attr(&mut node.attributes, "fillcolor", "yellow");
        add_attr(&mut node.attributes, "style", "filled");
        arena.visualize(&ctrl, &gen_id, self, &mut node.attributes);
        self.nodes.insert(index, node);
        gen_id
    }

    pub fn add_expr(&mut self, expr: &Rc<Expr>) -> String {
        let index = expr as *const _ as u64;
        if self.nodes.contains_key(&index) {
            return self.nodes[&index].id.0.to_string();
        }
        let gen_id = self.generate_id();
        let mut node = Node {
            id: NodeId(Id::Plain(gen_id.clone()), None),
            attributes: Vec::new(),
        };
        expr.visualize(&gen_id, self, &mut node.attributes);
        self.nodes.insert(index, node);
        gen_id
    }

    fn generate_graph(&self, title: &str, path: &Path) -> Result<()> {
        let mut stmts = Vec::<Stmt>::new();
        for node in self.nodes.values() {
            stmts.push(Stmt::Node(node.clone()));
        }
        for edge in self.edges.iter() {
            stmts.push(Stmt::Edge(edge.clone()));
        }
        stmts.push(Stmt::Attribute(Attribute(
            Id::Plain("rankdir".to_string()),
            Id::Plain("BT".to_string()),
        )));
        stmts.push(Stmt::Attribute(Attribute(
            Id::Plain("label".to_string()),
            Id::Plain(format!("\"{}\"", title)),
        )));
        let graph = Graph::DiGraph {
            id: Id::Plain("G".to_string()),
            strict: false,
            stmts: stmts,
        };
        let mut ctx = PrinterContext::default();
        let result = exec(graph, &mut ctx, vec![CommandArg::Format(Format::Svg)])
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let mut file = File::create(path).unwrap();
        file.write_all(&result).unwrap();
        Ok(())
    }
}

pub fn visualize(ctrl: &Rc<Ctrl>, title: &str, path: &Path) -> Result<()> {
    let mut visualizer = Visualizer::new();
    visualizer.add_ctrl(ctrl);
    visualizer.generate_graph(title, path)
}
