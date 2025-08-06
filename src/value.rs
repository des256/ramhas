use {crate::*, std::fmt::Display};

#[derive(Clone)]
pub struct Value {
    pub ty: Ty,
    pub data: Data,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}({})", self.ty, self.data)
        write!(f, "{}", self.data)
    }
}
