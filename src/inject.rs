use proc_macro2::TokenStream;
use syn::__private::quote;
use syn::{parse2, ItemFn, FnArg};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::provider::ProviderBuilder;

pub fn inject(
    attributes: TokenStream,
    body: TokenStream,
) -> TokenStream {
    let body: ItemFn = match parse2(body) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error(),
    };
    let attr: ProviderBuilder = match parse2(attributes) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error(),
    };
    let type_name = &attr.type_name;
    let sp_input: FnArg = match parse2(quote::quote! {sp: std::sync::Arc<#type_name>}) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error(),
    };
    let (visibility, attrs, block) = (&body.vis, &body.attrs, &body.block);
    let mut sig = body.sig.clone();
    let inputs = sig.inputs;
    let mut new_inputs = Punctuated::<FnArg, Comma>::new();
    let mut injects = Vec::<TokenStream>::new();
    let header = if attr.is_async {
        attr.scoped_config.map(|x| {
            quote::quote! {
                let sp = sp.create_scoped(#x).await.unwrap();
            }
        })
    } else {
        attr.scoped_config.map(|x| {
            quote::quote! {
                let sp = sp.create_scoped(#x).unwrap();
            }
        })
    };
    for input in inputs.iter() {
        match input {
            FnArg::Typed(x) => {
                if x.attrs
                    .iter()
                    .any(|x| x.path().get_ident().unwrap().to_string().eq("inject"))
                {
                    let (pat, ty) = (&x.pat, &x.ty);
                    injects.push(quote::quote! {
                        let #pat: #ty = sp.provide();
                    });
                } else {
                    new_inputs.push(FnArg::Typed(x.clone()))
                }
            }
            _ => new_inputs.push(input.clone()),
        }
    }
    new_inputs.push(sp_input);
    sig.inputs = new_inputs;
    quote::quote! {
        #(#attrs)*
        #visibility #sig {
            #header
            #(#injects)*
            #block
        }
    }
}
