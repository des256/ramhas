use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub enum BoolValue {
    All,
    Constant(bool),
    Any,
}

impl BoolValue {
    pub fn meet(&self, other: &BoolValue) -> BoolValue {
        match (self, other) {
            (BoolValue::All, _) => other.clone(),
            (_, BoolValue::All) => self.clone(),
            (BoolValue::Constant(a), BoolValue::Constant(b)) => {
                if a == b {
                    BoolValue::Constant(*a)
                } else {
                    BoolValue::Any
                }
            }
            (BoolValue::Any, _) => BoolValue::Any,
            (_, BoolValue::Any) => BoolValue::Any,
        }
    }

    pub fn join(&self, other: &BoolValue) -> BoolValue {
        match (self, other) {
            (BoolValue::All, _) => BoolValue::All,
            (_, BoolValue::All) => BoolValue::All,
            (BoolValue::Constant(a), BoolValue::Constant(b)) => {
                if a == b {
                    BoolValue::Constant(*a)
                } else {
                    BoolValue::Any
                }
            }
            (BoolValue::Any, _) => other.clone(),
            (_, BoolValue::Any) => self.clone(),
        }
    }
}

impl Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoolValue::All => write!(f, "bool(all)"),
            BoolValue::Constant(a) => write!(f, "{}", a),
            BoolValue::Any => write!(f, "bool(any)"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum IntValue {
    All,
    Constant(i64),
    Any,
}

impl IntValue {
    pub fn meet(&self, other: &IntValue) -> IntValue {
        match (self, other) {
            (IntValue::All, _) => other.clone(),
            (_, IntValue::All) => self.clone(),
            (IntValue::Constant(a), IntValue::Constant(b)) => {
                if a == b {
                    IntValue::Constant(*a)
                } else {
                    IntValue::Any
                }
            }
            (IntValue::Any, _) => IntValue::Any,
            (_, IntValue::Any) => IntValue::Any,
        }
    }

    pub fn join(&self, other: &IntValue) -> IntValue {
        match (self, other) {
            (IntValue::All, _) => IntValue::All,
            (_, IntValue::All) => IntValue::All,
            (IntValue::Constant(a), IntValue::Constant(b)) => {
                if a == b {
                    IntValue::Constant(*a)
                } else {
                    IntValue::Any
                }
            }
            (IntValue::Any, _) => other.clone(),
            (_, IntValue::Any) => self.clone(),
        }
    }
}

impl Display for IntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntValue::All => write!(f, "int(all)"),
            IntValue::Constant(a) => write!(f, "{}", a),
            IntValue::Any => write!(f, "int(any)"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Value {
    All,
    Bool(BoolValue),
    Int(IntValue),
    Any,
}

impl Value {
    pub fn meet(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::All, _) => other.clone(),
            (_, Value::All) => self.clone(),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a.meet(b)),
            (Value::Int(a), Value::Int(b)) => Value::Int(a.meet(b)),
            (Value::Any, _) => Value::Any,
            (_, Value::Any) => Value::Any,
            (_, _) => panic!("unable to meet {} with {}", self, other),
        }
    }

    pub fn join(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::All, _) => Value::All,
            (_, Value::All) => Value::All,
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a.join(b)),
            (Value::Int(a), Value::Int(b)) => Value::Int(a.join(b)),
            (Value::Any, _) => other.clone(),
            (_, Value::Any) => self.clone(),
            (_, _) => panic!("unable to join {} with {}", self, other),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::All => write!(f, "all"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Any => write!(f, "any"),
        }
    }
}

/*
#[derive(Clone)]
pub enum Data {
    Bool(bool),
    Int(i64),
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Bool(value) => write!(f, "{}", value),
            Data::Int(value) => write!(f, "{}", value),
        }
    }
}
*/
