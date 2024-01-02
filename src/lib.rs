//! An efficient bioinformatics file parser based on memory mapping of file.

#![warn(missing_docs)]

/* std use */

/* crate use */

/* project use */
#[cfg(feature = "derive")]
pub use biommap_derive::{self, *};

/* mod declaration */
pub mod block;

pub mod error;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "fasta")]
pub mod fasta;

#[cfg(feature = "fastq")]
pub mod fastq;

/// Define default blocksize
pub const DEFAULT_BLOCKSIZE: u64 = 8192;
