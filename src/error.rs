//! Error struct of project biommap

/* crate use */
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    /// Missing metadata file
    #[error("biommap failled to read file metadata {source}")]
    MetaDataFile {
        /// Original error
        source: std::io::Error,
    },

    /// Can't open file
    #[error("biommap can't open file {source}")]
    OpenFile {
        /// Original error
        source: std::io::Error,
    },

    /// Can't map file in memory
    #[error("biommap can't map file on memory {source}")]
    MapFile {
        /// Original error
        source: std::io::Error,
    },

    /// biommap didn't find a new line in block, extend block size could by a solution
    #[error("biommap didn't find new line in block increase block size")]
    NoNewLineInBlock,

    /// File seems not containts fastq data
    #[error("Input file seems not be a fastq file")]
    NotAFastqFile,

    /// File seems not containts fasta data
    #[error("Input file seems not be a fasta file")]
    NotAFastaFile,

    /// Current record seems to be a partial record
    #[error("biommap found a partial record")]
    PartialRecord,
}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
