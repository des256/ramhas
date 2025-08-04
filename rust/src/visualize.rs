use {
    crate::*,
    anyhow::Result,
    graphviz_rust::{
        cmd::{CommandArg, Format},
        dot_structures::{Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Port, Stmt, Vertex},
        exec, print,
        printer::PrinterContext,
    },
    std::{cell::RefCell, collections::HashMap, fs::File, io::Write, path::Path, rc::Rc},
};

pub(crate) struct Visualizer {
    next_id: usize,
    nodes: HashMap<u64, Node>,
    edges: Vec<Edge>,
}

pub fn add_attr(attributes: &mut Vec<Attribute>, name: &str, value: &str) {
    attributes.push(Attribute(
        Id::Plain(name.to_string()),
        Id::Plain(value.to_string()),
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
                Vertex::N(NodeId(Id::Plain(from.to_string()), None)),
                Vertex::N(NodeId(Id::Plain(to.to_string()), None)),
            ),
            attributes: if red {
                vec![Attribute(
                    Id::Plain("color".to_string()),
                    Id::Plain("red".to_string()),
                )]
            } else {
                Vec::new()
            },
        });
    }

    pub fn add_n2p(&mut self, from: &str, to: &str, port: &str, red: bool) {
        self.edges.push(Edge {
            ty: EdgeTy::Pair(
                Vertex::N(NodeId(Id::Plain(from.to_string()), None)),
                Vertex::N(NodeId(
                    Id::Plain(to.to_string()),
                    Some(Port(Some(Id::Plain(port.to_string())), None)),
                )),
            ),
            attributes: if red {
                vec![Attribute(
                    Id::Plain("color".to_string()),
                    Id::Plain("red".to_string()),
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
                    Id::Plain(from.to_string()),
                    Some(Port(Some(Id::Plain(port.to_string())), None)),
                )),
                Vertex::N(NodeId(Id::Plain(to.to_string()), None)),
            ),
            attributes: if red {
                vec![Attribute(
                    Id::Plain("color".to_string()),
                    Id::Plain("red".to_string()),
                )]
            } else {
                Vec::new()
            },
        });
    }

    pub fn add_scopes(&mut self, gen_id: &str, scopes: &Scopes) -> String {
        let index = (&*scopes as *const Scopes) as u64;
        if self.nodes.contains_key(&index) {
            return self.nodes[&index].id.0.to_string();
        }
        let scopes_id = self.generate_id();
        let mut node = Node {
            id: NodeId(Id::Plain(scopes_id.clone()), None),
            attributes: Vec::new(),
        };
        add_attr(&mut node.attributes, "shape", "record");
        let mut label = "\"{Scopes".to_string();
        for (i, bindings) in scopes.bindings.iter().enumerate() {
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

    pub fn add_ctrl(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> String {
        let index = ctrl as *const _ as u64;
        if self.nodes.contains_key(&index) {
            return self.nodes[&index].id.0.to_string();
        }
        let gen_id = self.generate_id();
        let mut node = Node {
            id: NodeId(Id::Plain(gen_id.clone()), None),
            attributes: Vec::new(),
        };
        add_attr(&mut node.attributes, "shape", "record");
        add_attr(&mut node.attributes, "fillcolor", "yellow");
        add_attr(&mut node.attributes, "style", "filled");
        ctrl.borrow_mut()
            .visualize(&gen_id, self, &mut node.attributes);
        self.nodes.insert(index, node);
        gen_id
    }

    pub fn add_expr(&mut self, expr: &Rc<RefCell<dyn Expr>>) -> String {
        let index = expr as *const _ as u64;
        if self.nodes.contains_key(&index) {
            return self.nodes[&index].id.0.to_string();
        }
        let gen_id = self.generate_id();
        let mut node = Node {
            id: NodeId(Id::Plain(gen_id.clone()), None),
            attributes: Vec::new(),
        };
        match &*expr.borrow() {
            Expr::Proj { ctrl, index } => {
                add_attr(
                    &mut node.attributes,
                    "label",
                    &format!("\"Proj {}\"", index),
                );
                let ctrl_id = self.add_ctrl(ctrl);
                self.add_n2n(&gen_id, &ctrl_id, true);
            }
            Expr::Constant { value } => {
                add_attr(&mut node.attributes, "label", &format!("\"{}\"", value));
            }
            Expr::Add { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"+\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Sub { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"-\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Mul { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"*\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Div { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"/\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Neg { expr } => {
                add_attr(&mut node.attributes, "label", "\"-\"");
                let expr_id = self.add_expr(expr);
                self.add_n2n(&gen_id, &expr_id, false);
            }
            Expr::Not { expr } => {
                add_attr(&mut node.attributes, "label", "\"!\"");
                let expr_id = self.add_expr(expr);
                self.add_n2n(&gen_id, &expr_id, false);
            }
            Expr::Equal { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"==\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::NotEqual { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"!=\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LessThan { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"<\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LessThanOrEqual { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\"<=\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::GreaterThan { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\">\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::GreaterThanOrEqual { lhs, rhs } => {
                add_attr(&mut node.attributes, "label", "\">=\"");
                let lhs_id = self.add_expr(lhs);
                let rhs_id = self.add_expr(rhs);
                self.add_n2n(&gen_id, &lhs_id, false);
                self.add_n2n(&gen_id, &rhs_id, false);
            }
        }
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

pub fn visualize(ctrl: &Rc<RefCell<Ctrl>>, title: &str, path: &Path) -> Result<()> {
    let mut visualizer = Visualizer::new();
    visualizer.add_ctrl(ctrl);
    visualizer.generate_graph(title, path)
}
