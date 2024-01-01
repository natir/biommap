//! Block related macro

/* std use */

/* crate use */

/* project use */

#[derive(Default)]
pub struct File2BlockAttributes {
    pub name: Option<syn::Ident>,
    pub block_type: Option<syn::Type>,
}

impl File2BlockAttributes {
    pub fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("name") {
            self.name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("block_type") {
            self.block_type = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported file block producer"))
        }
    }
}

pub fn file2block_quote(
    name: Option<syn::Ident>,
    block_type: Option<syn::Type>,
    method: syn::ItemFn,
) -> proc_macro2::TokenStream {
    quote::quote! {
    /// Block producer
    pub struct #name {
        offset: u64,
        blocksize: u64,
        file: std::fs::File,
        file_length: u64,
    }

    impl #name {
        /// Create a new Block producer
            #[inline(always)]
            pub fn new<P>(path: P) -> error::Result<Self>
            where
                P: AsRef<std::path::Path>,
                Self: Sized,
            {
                Self::with_blocksize(crate::DEFAULT_BLOCKSIZE, path)
            }

            /// Create a new Block producer with a blocksize choose by user
            pub fn with_blocksize<P>(blocksize: u64, path: P) -> error::Result<Self>
            where
                P: AsRef<std::path::Path>,
            {
                Ok(Self {
                    offset: 0,
                    blocksize: Self::fix_blocksize::<P>(&path, blocksize)?,
                    file_length: Self::filesize::<P>(&path)?,
                    file: std::fs::File::open(path)
                        .map_err(|source| error::Error::OpenFile { source })?,
                })
            }

            /// Create a new Block producer with offset choose by user
            pub fn with_offset<P>(offset: u64, path: P) -> error::Result<Self>
            where
                P: AsRef<std::path::Path>,
            {
                Ok(Self {
                    offset: offset,
                    blocksize: Self::fix_blocksize::<P>(&path, crate::DEFAULT_BLOCKSIZE)?,
                    file_length: Self::filesize::<P>(&path)?,
                    file: std::fs::File::open(path)
                        .map_err(|source| error::Error::OpenFile { source })?,
                })
            }

            /// Create a new Block producer with offset choose by user
            pub fn with_blocksize_offset<P>(
                blocksize: u64,
                offset: u64,
                path: P,
            ) -> error::Result<Self>
            where
                P: AsRef<std::path::Path>,
            {
                Ok(Self {
                    offset: offset,
                    blocksize: Self::fix_blocksize::<P>(&path, blocksize)?,
                    file_length: Self::filesize::<P>(&path)?,
                    file: std::fs::File::open(path)
                        .map_err(|source| error::Error::OpenFile { source })?,
                })
            }

            /// Get next block
            pub fn next_block(&mut self) -> error::Result<Option<block::Block<#block_type>>> {
                if self.offset() == self.file_length() {
                    Ok(None)
                } else if self.offset() + self.blocksize() >= self.file_length() {
                    let data = unsafe {
                        memmap2::MmapOptions::new()
                            .offset(self.offset())
                            .len((self.file_length() - self.offset()) as usize)
                            .map(self.file())
                            .map_err(|source| error::Error::MapFile { source })?
                    };

                    self.set_offset(self.file_length());
            let data_len = data.len() as u64;

                    Ok(Some(block::Block::new(data, data_len)))
                } else {
                    let data = unsafe {
                        memmap2::MmapOptions::new()
                            .offset(self.offset())
                            .len(self.blocksize() as usize)
                            .map(self.file())
                            .map_err(|source| error::Error::MapFile { source })?
                    };

                    let blocksize = Self::correct_block_size(&data)?;
                    self.set_offset(self.offset() + blocksize);
                    Ok(Some(block::Block::new(data, blocksize)))
                }
            }

            /// Get file size
            pub fn filesize<P>(path: &P) -> error::Result<u64>
            where
                P: AsRef<std::path::Path>,
            {
                Ok(path
                    .as_ref()
                    .metadata()
                    .map_err(|source| error::Error::MetaDataFile { source })?
                    .len() as u64)
            }

            /// Fix blocksize to file size in file size is lower than blocksize
            pub fn fix_blocksize<P>(path: &P, blocksize: u64) -> error::Result<u64>
            where
                P: AsRef<std::path::Path>,
                Self: Sized,
            {
                Ok(Self::filesize::<P>(path)?.min(blocksize) as u64)
            }

    /// Search the begin of the partial record at the end of #name [Block](Block)
            #method

            /// Get current value of offset
            pub fn offset(&self) -> u64 {
                self.offset
            }

            /// Get file length
            pub fn file_length(&self) -> u64 {
                self.file_length
            }

            /// Get file
            pub fn file(&self) -> &std::fs::File {
                &self.file
            }

            /// Get blocksize
            pub fn blocksize(&self) -> u64 {
                self.blocksize
            }

            /// Set value of offset
            pub fn set_offset(&mut self, value: u64) {
                self.offset = value;
    }

        }

        impl Iterator for #name {
            type Item = error::Result<block::Block<#block_type>>;

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

#[derive(Default)]
pub struct Block2RecordAttributes {
    pub name: Option<syn::Ident>,
    pub block_type: Option<syn::Type>,
}

impl Block2RecordAttributes {
    pub fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
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

pub fn block2record_quote(
    name: Option<syn::Ident>,
    generic_type: Option<syn::Type>,
    method: syn::ItemFn,
) -> proc_macro2::TokenStream {
    quote::quote! {
    /// $name is a reader struct
        pub struct #name<#generic_type> {
            offset: u64,
            block: block::Block<#generic_type>,
        }

        impl<#generic_type> #name<#generic_type>
    where
        #generic_type: core::convert::AsRef<[u8]>,
    {
            /// Create a new $name
            pub fn new(block: block::Block<#generic_type>) -> Self {
                Self { offset: 0, block }
            }

            /// Get the next available record
            #method

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
