//! Proc macro crate of biommap crate.

#![warn(missing_docs)]

/* std use */

/* crate use */

/* project use */

/* mod declaration */
mod block;

/// Macro to create a file block producer from correct_block_size member function
#[proc_macro_attribute]
pub fn file2block(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut attrs = block::File2BlockAttributes::default();
    let attrs_parser = syn::meta::parser(|meta| attrs.parse(meta));
    syn::parse_macro_input!(args with attrs_parser);

    let mut method = syn::parse_macro_input!(input as syn::ItemFn);
    method.sig.ident = syn::parse_str::<syn::Ident>("correct_block_size").unwrap();

    let token = block::file2block_quote(attrs.name, attrs.block_type, method);

    proc_macro::TokenStream::from(token)
}

/// Macro to create a block parser from next record member function
#[proc_macro_attribute]
pub fn block2record(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut attrs = block::Block2RecordAttributes::default();
    let attrs_parser = syn::meta::parser(|meta| attrs.parse(meta));
    syn::parse_macro_input!(args with attrs_parser);

    let mut method = syn::parse_macro_input!(input as syn::ItemFn);
    method.sig.ident = syn::parse_str::<syn::Ident>("next_record").unwrap();

    let token = block::block2record_quote(attrs.name, attrs.block_type, method);

    proc_macro::TokenStream::from(token)
}