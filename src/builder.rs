use crate::dependency::DependencyBuilder;
use syn::{parse::Parse, Attribute, ExprBlock, Field, Ident, Visibility};

#[derive(Clone, Debug)]
pub(crate) struct ContainerBuilder {
    pub name: Ident,
    pub metas: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub dependencies: Vec<DependencyBuilder>,
    pub struct_params: Vec<Field>,
    pub scoped_struct_params: Vec<Field>,
    pub after_build: Option<ExprBlock>,
}
