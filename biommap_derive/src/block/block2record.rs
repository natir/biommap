//! Block related macro

/* std use */

/* crate use */

/* project use */

#[derive(Default)]
pub(crate) struct Attributes {
    pub name: Option<syn::Ident>,
    pub block_type: Option<syn::Type>,
}

impl Attributes {
    pub(crate) fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("name") {
            self.name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("generic_type") {
            self.block_type = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported file block producer"))
        }
    }
}

fn quote_struct(
    name: Option<syn::Ident>,
    generic_type: Option<syn::Type>,
) -> proc_macro2::TokenStream {
    quote::quote! {
    /// #name is a reader struct
        pub struct #name<#generic_type> {
            offset: u64,
            block: block::Block<#generic_type>,
        }
    }
}

fn quote_base_impl(
    name: Option<syn::Ident>,
    generic_type: Option<syn::Type>,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl<#generic_type> #name<#generic_type>
        where
            #generic_type: core::convert::AsRef<[u8]>,
        {
                /// Create a new #name
                pub fn new(block: block::Block<#generic_type>) -> Self {
                    Self { offset: 0, block }
                }

                /// A utils function to get range of the next line
                pub fn get_line(&self) -> error::Result<std::ops::Range<usize>> {
                    let next = memchr::memchr(b'\n', &self.block.data()[self.offset as usize..])
                        .ok_or(error::Error::PartialRecord)?;
                    let range = self.offset as usize..self.offset as usize + next;

                    Ok(range)
                }
            }
    }
}

pub(crate) fn quote(
    name: Option<syn::Ident>,
    generic_type: Option<syn::Type>,
    method: syn::ItemFn,
) -> proc_macro2::TokenStream {
    let struct_token = quote_struct(name.clone(), generic_type.clone());
    let impl_token = quote_base_impl(name.clone(), generic_type.clone());

    quote::quote! {
    #struct_token

    #impl_token

        impl<#generic_type> #name<#generic_type>
        where
            #generic_type: core::convert::AsRef<[u8]>,
        {
                /// Get the next available record
                #method
    }
    }
}
