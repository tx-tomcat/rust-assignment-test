use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, Lit, Meta, NestedMeta, Pat};

#[proc_macro_attribute]
pub fn cached(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut cache_time = None;
    let cache_field_name = parse_cache_field_name(&args);
    let cache_field = format_ident!("{}", cache_field_name);
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("cache_time") => {
                if let Lit::Int(lit) = nv.lit {
                    cache_time = Some(lit);
                }
            }

            _ => {}
        }
    }
    let cache_time = cache_time.expect("cache_time must be specified");
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_body = &input_fn.block;
    let param_name = fn_sig
        .inputs
        .iter()
        .find_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some(&pat_ident.ident)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("Expected at least one parameter");

    quote! {
        #fn_vis #fn_sig {
            use std::time::Duration;
            let key = (#param_name,);
            let ttl = Duration::from_secs(#cache_time);

            self.#cache_field.get_or_insert_with(key, ttl, || async {
                #fn_body
            }).await
        }
    }
    .into()
}

fn parse_cache_field_name(args: &AttributeArgs) -> String {
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("cache_field_name") {
                if let Lit::Str(lit) = &nv.lit {
                    return lit.value();
                }
            }
        }
    }
    "cache".to_string()
}
