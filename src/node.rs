use {generational_arena::Index as GenIndex, std::fmt::Display};

pub enum Node {
    Start,
    Return { ctrl: GenIndex, id: GenIndex },
    Constant { value: i64 },
    Add { id: GenIndex, id2: GenIndex },
    Sub { id: GenIndex, id2: GenIndex },
    Mul { id: GenIndex, id2: GenIndex },
    Div { id: GenIndex, id2: GenIndex },
    Neg { id: GenIndex },
}

impl Node {
    pub fn equals(&self, other: &Node) -> bool {
        match (self, other) {
            (Node::Start, Node::Start) => true,
            (
                Node::Return {
                    ctrl: ctrl_a,
                    id: id_a,
                },
                Node::Return {
                    ctrl: ctrl_b,
                    id: id_b,
                },
            ) => (ctrl_a == ctrl_b) && (id_a == id_b),
            (Node::Constant { value: value_a }, Node::Constant { value: value_b }) => {
                value_a == value_b
            }
            (
                Node::Add {
                    id: id_a,
                    id2: id2_a,
                },
                Node::Add {
                    id: id_b,
                    id2: id2_b,
                },
            ) => (id_a == id_b) && (id2_a == id2_b),
            (
                Node::Sub {
                    id: id_a,
                    id2: id2_a,
                },
                Node::Sub {
                    id: id_b,
                    id2: id2_b,
                },
            ) => (id_a == id_b) && (id2_a == id2_b),
            (
                Node::Mul {
                    id: id_a,
                    id2: id2_a,
                },
                Node::Mul {
                    id: id_b,
                    id2: id2_b,
                },
            ) => (id_a == id_b) && (id2_a == id2_b),
            (
                Node::Div {
                    id: id_a,
                    id2: id2_a,
                },
                Node::Div {
                    id: id_b,
                    id2: id2_b,
                },
            ) => (id_a == id_b) && (id2_a == id2_b),
            (Node::Neg { id: id_a }, Node::Neg { id: id_b }) => id_a == id_b,
            _ => false,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Start => write!(f, "Start"),
            Node::Return { ctrl, id } => {
                write!(f, "Return({:?} from {:?})", id, ctrl)
            }
            Node::Constant { value } => write!(f, "Constant({})", value),
            Node::Add { id, id2 } => {
                write!(f, "Add({:?},{:?})", id, id2)
            }
            Node::Sub { id, id2 } => {
                write!(f, "Sub({:?},{:?})", id, id2)
            }
            Node::Mul { id, id2 } => {
                write!(f, "Mul({:?},{:?})", id, id2)
            }
            Node::Div { id, id2 } => {
                write!(f, "Div({:?},{:?})", id, id2)
            }
            Node::Neg { id } => {
                write!(f, "Neg({:?})", id)
            }
        }
    }
}
