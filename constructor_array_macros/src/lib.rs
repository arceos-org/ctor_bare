//！Macros for registering constructor functions for Rust under no_std, which is like __attribute__((constructor)) in C/C++.
//!
//! **DO NOT** use this crate directly. Use the [constructor_array](https://docs.rs/constructor_array) crate instead.
//!
//! After attching the `register_ctor` macro to the given function, a pointer pointing to it will be stored in the `ctors` section.
//! When the program is loaded, this section will be linked into the binary. The `invoke_ctors` function in the `constructor_array`
//! crate will call all the constructor functions in the `ctors` section.
//!
//! See the documentation of the [constructor_array](https://docs.rs/constructor_array) crate for more details.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Error, Item};

/// Register a constructor function to be called before `main`.
///
/// The function should have no input arguments and return nothing.
///
/// See the documentation of the [constructor_array](https://docs.rs/constructor_array) crate for more details.
#[proc_macro_attribute]
pub fn register_ctor(attr: TokenStream, function: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return Error::new(
            Span::call_site(),
            "expect an empty attribute: `#[register_ctor]`",
        )
        .to_compile_error()
        .into();
    }

    let item: Item = parse_macro_input!(function as Item);
    if let Item::Fn(func) = item {
        let name = &func.sig.ident;
        let name_str = name.to_string();
        let name_ident = format_ident!("_CTOR_{}", name_str);
        let output = &func.sig.output;
        // Constructor functions should not have any return value.
        if let syn::ReturnType::Type(_, _) = output {
            return Error::new(
                Span::call_site(),
                "expect no return value for the constructor function",
            )
            .to_compile_error()
            .into();
        }
        let inputs = &func.sig.inputs;
        // Constructor functions should not have any input arguments.
        if !inputs.is_empty() {
            return Error::new(
                Span::call_site(),
                "expect no input arguments for the constructor function",
            )
            .to_compile_error()
            .into();
        }
        let block = &func.block;

        quote! {
            #[link_section = "ctors"]
            #[allow(non_upper_case_globals)]
            static #name_ident: extern "C" fn() = #name;

            #[no_mangle]
            #[allow(non_upper_case_globals)]
            pub extern "C" fn #name() {
                #block
            }
        }
        .into()
    } else {
        Error::new(Span::call_site(), "expect a function to be registered")
            .to_compile_error()
            .into()
    }
}
