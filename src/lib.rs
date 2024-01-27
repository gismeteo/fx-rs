mod builder;
mod dependency;
mod inject;
mod provider;

use crate::builder::ContainerBuilder;
use crate::inject::inject;
use proc_macro::TokenStream;
use syn::parse2;

#[proc_macro]
pub fn build_container(body: TokenStream) -> TokenStream {
    internal_build_container(proc_macro2::TokenStream::from(body)).into()
}

#[proc_macro_attribute]
pub fn auto_inject(attr: TokenStream, body: TokenStream) -> TokenStream {
    inject(
        proc_macro2::TokenStream::from(attr),
        proc_macro2::TokenStream::from(body),
    )
    .into()
}

fn internal_build_container(body: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input = match parse2::<ContainerBuilder>(body) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error(),
    };
    input.build()
}
