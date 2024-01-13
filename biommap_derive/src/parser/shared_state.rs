//! sequential related function and struct

/* std use */

/* crate use */

/* project use */

#[derive(Default)]
pub(crate) struct Attributes {
    pub name: Option<syn::Ident>,
    pub data_type: Option<syn::Type>,
    pub block_producer: Option<syn::Type>,
    pub record_producer: Option<syn::Type>,
}

impl Attributes {
    pub(crate) fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("name") {
            self.name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("data_type") {
            self.data_type = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("block_producer") {
            self.block_producer = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("record_producer") {
            self.record_producer = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported file block producer"))
        }
    }
}

fn quote_struct(name: Option<syn::Ident>) -> proc_macro2::TokenStream {
    quote::quote! {
    /// Shared state parser
    pub struct #name {}
    }
}

fn quote_impl(
    name: Option<syn::Ident>,
    data_type: Option<syn::Type>,
    block_producer: Option<syn::Type>,
    record_producer: Option<syn::Type>,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #name {
        /// Create a new #name
            pub fn new() -> Self {
                Self {}
            }

        /// Parse file point on path
            pub fn parse<P>(&mut self, path: P, data: &#data_type) -> biommap::error::Result<()>
            where
                P: AsRef<std::path::Path>,
            {
                self.with_blocksize(biommap::DEFAULT_BLOCKSIZE, path, data)
            }

        /// Parse file point on path with a specif blocksize
            pub fn with_blocksize<P>(
                &mut self,
                blocksize: u64,
                path: P,
                data: &#data_type,
            ) -> biommap::error::Result<()>
            where
                P: AsRef<std::path::Path>,
            {
        let producer = #block_producer::with_blocksize(blocksize, path)?;

                match producer
                    .par_bridge()
                    .map(|block| {
                        let mut reader = #record_producer::new(block?);
                        while let Some(record) = reader.next_record()? {
                            Self::record(record, data);
                        }
                        Ok(())
                    })
                    .find_any(|x| x.is_err())
                {
                    Some(e) => e,
                    None => Ok(()),
                }
            }
    }
    }
}

pub fn quote(
    name: Option<syn::Ident>,
    data_type: Option<syn::Type>,
    block_producer: Option<syn::Type>,
    record_producer: Option<syn::Type>,
    method: syn::ItemFn,
) -> proc_macro2::TokenStream {
    let struct_token = quote_struct(name.clone());
    let impl_token = quote_impl(
        name.clone(),
        data_type.clone(),
        block_producer.clone(),
        record_producer.clone(),
    );

    quote::quote! {
    #struct_token

    #impl_token

    impl #name {
        #method
    }
    }
}
