use syn::{parse::Parse, Attribute, ExprAsync, ExprBlock, Field, Type};

#[derive(Clone, Debug)]
pub(crate) struct DependencyBuilder {
    pub metas: Vec<Attribute>,
    pub field: Field,
    pub build: Option<ExprBlock>,
    pub build_async: Option<ExprAsync>,
    pub provides: Vec<Type>,
    pub dependency_type: DependencyType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DependencyType {
    Outter,
    Singleton,
    Scoped,
    Transient,
}
