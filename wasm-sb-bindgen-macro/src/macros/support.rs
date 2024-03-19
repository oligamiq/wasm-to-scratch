use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Ident};

pub fn expand(_attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<syn::Item>(input)?;

    let ast = match item {
        syn::Item::Fn(item) => item,
        _ => abort!(item, "only functions are supported"),
    };

    // dbg!(&ast);

    let fn_ast = ast.clone();
    let call_fn_name = fn_ast.sig.ident.clone();
    let fn_generic = fn_ast.sig.generics.clone();
    let fn_name = fn_ast.sig.ident.to_string();
    let fn_ast = match syn::parse2::<syn::Item>(quote! {
        #[allow(dead_code)]
        #fn_ast
    })? {
        syn::Item::Fn(item) => item,
        _ => unreachable!(),
    };

    // export wasm glue
    let wrapper_fn_name: Ident =
        syn::parse_str(&(String::from("__wasm_sb_bindgen_generated_") + &fn_name))?;
    // let wrapper_fn = quote! {
    //     #[automatically_derived]
    //     const _: () = {
    //         #[export_name = #fn_name]
    //         pub unsafe extern "C" fn #wrapper_fn_name(
    //             arg0_ptr: *const u8,
    //             arg0_len: usize,
    //         ) -> *mut String {
    //             let arg0 = {
    //                 let slice = ::std::slice::from_raw_parts(arg0_ptr, arg0_len);
    //                 ::std::str::from_utf8_unchecked(slice)
    //             };
    //             let _ret = #call_fn_name(arg0.to_string());
    //             Box::into_raw(Box::new(_ret))
    //         }
    //     };
    // };

    let inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = &fn_ast.sig.inputs;
    // dbg!(&fn_ast);
    // dbg!(&inputs);

    let inputs = inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => Err(anyhow::anyhow!("receiver not supported")),
            syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(_) => Ok(pat_type.ty.clone()),
                syn::Pat::Tuple(_) => Err(anyhow::anyhow!("tuple not supported")),
                _ => Err(anyhow::anyhow!("unsupported pattern")),
            },
        })
        .collect::<Result<Vec<_>>>()?;

    let wrapper_fn_inputs = inputs.iter().enumerate().flat_map(|(index, ty)| {
        let mut idents = Vec::new();
        for k in 1..5 {
            let ident = syn::parse_str::<Ident>(&format!("arg{index}_{k}")).unwrap();
            let out_ty = syn::parse_str::<syn::Type>(&format!("Prim{k}")).unwrap();
            let ident = quote! { #ident: <<#ty as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::#out_ty };
            let ident = syn::parse2::<syn::FnArg>(ident).unwrap();
            idents.push(ident);
        }
        idents
    }).collect::<Punctuated<syn::FnArg, syn::token::Comma>>();

    let wrapper_fn_outputs = syn::parse2::<syn::Type>(match &fn_ast.sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    })
    .unwrap();

    let from_abi_fn = inputs.iter().enumerate().map(|(index, ty)| {
        let arg_ident = syn::parse_str::<Ident>(&format!("arg{index}")).unwrap();
        let idents = (1..5).map(|k| {
            syn::parse_str::<Ident>(&format!("arg{index}_{k}")).unwrap()
        }).collect::<Punctuated<Ident, Comma>>();
        let arg = quote! {
            let #arg_ident = unsafe {
                <#ty as wasm_sb_bindgen::convert::FromWasmAbi>::from_abi(
                    <<#ty as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::join(
                        #idents
                    ),
                )
            };
        };
        arg
    }).fold(quote! {}, |acc, f| quote! { #acc #f } );

    let call_fn_inputs = (0..inputs.len())
        .map(|index| syn::parse_str::<Ident>(&format!("arg{index}")).unwrap())
        .collect::<Punctuated<Ident, Comma>>();

    let mut wrapper_fn_inner = match syn::parse2::<syn::Item>(quote! {
        #[export_name = #fn_name]
        pub unsafe extern "C" fn #wrapper_fn_name(
            #wrapper_fn_inputs
        ) -> wasm_sb_bindgen::convert::WasmRet<
            <#wrapper_fn_outputs as wasm_sb_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                #from_abi_fn
                let _ret = #call_fn_name(
                    #call_fn_inputs
                );
                _ret
            };
            <#wrapper_fn_outputs as wasm_sb_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
        }
    })? {
        syn::Item::Fn(item) => item,
        _ => unreachable!(),
    };

    wrapper_fn_inner.sig.generics = fn_generic;

    let wrapper_fn = quote! {
        #[automatically_derived]
        const _: () = {
            #wrapper_fn_inner
        };
    };

    let describe_fn_name: Ident =
        syn::parse_str(&(String::from("__wasm_sb_bindgen_describe_") + &fn_name))?;

    let describe_fn = quote! {
        #[automatically_derived]
        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn #describe_fn_name() {
                use wasm_sb_bindgen::describe::*;
                wasm_sb_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION)
            }
        };
    };

    let gen = quote! {
        #fn_ast
        #wrapper_fn
        #describe_fn
    };

    Ok(gen)
}
