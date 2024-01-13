//! Block related macro

/* std use */

/* crate use */

/* project use */

/* mod declaration */
pub mod block2record;
pub mod file2block;

pub(crate) fn impl_iterator(
    name: Option<syn::Ident>,
    iterator_type: Option<syn::Type>,
) -> proc_macro2::TokenStream {
    quote::quote! {
    impl Iterator for #name {
            type Item = error::Result<#iterator_type>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.next_block() {
                    Ok(Some(block)) => Some(Ok(block)),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                }
            }
    }
    }
}
