use proc_macro2::TokenStream;
use syn::Type;

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    async fn run(&self);
}

#[derive(Clone, Debug)]
pub(crate) struct ProviderBuilder {
    pub type_name: Type,
    pub scoped_config: Option<TokenStream>,
    pub is_async: bool,
}