extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_str, FnArg, Ident, ItemTrait, ReturnType, TraitItem, TraitItemFn};

/// Create interface trait suitable for usage in integration tests
#[proc_macro_attribute]
pub fn make_integration_version(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as ItemTrait);

    let trait_name = &input.ident;

    let async_trait_name = Ident::new(&format!("{trait_name}Integration"), trait_name.span());

    let async_methods = input.items.iter().filter_map(|item| {
        if let TraitItem::Fn(method) = item {
            let async_method = asyncify_method(method.clone());
            Some(async_method)
        } else {
            None
        }
    });

    quote! {

        #input

        #[async_trait::async_trait]
        pub trait #async_trait_name {
            #(#async_methods)*
        }
    }
    .into()
}

fn asyncify_method(mut method: TraitItemFn) -> proc_macro2::TokenStream {
    method.sig.asyncness = Some(Default::default());

    let mut ret = if matches!(method.sig.output, ReturnType::Default) {
        "()".to_string()
    } else {
        let ret = method.sig.output.to_token_stream().to_string();

        let ret = ret.strip_prefix("-> ").unwrap();

        ret.to_string()
    };

    if ret == "Self" {
        let self_arg: FnArg = parse_str("&self").unwrap();
        method.sig.inputs.insert(0, self_arg);
        ret = "()".to_string();
    }

    if ret.starts_with("PromiseOrValue <") {
        let start = ret.find('<').unwrap();
        let end = ret.find('>').unwrap();

        ret = ret[start + 1..end].to_string();
    }

    let ret: Result<ReturnType, _> = parse_str(&format!("-> anyhow::Result<{ret}>"));

    method.sig.output = ret.unwrap();

    method.to_token_stream()
}
