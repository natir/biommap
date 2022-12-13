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

    /// Generic io::Error
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// biommap didn't find a new line in block, extend block size could by a solution
    #[error("biommap didn't find new line in block increase block size")]
    NoNewLineInBlock,

    /// File seems not containts fastq data
    #[error("Input file seems not be a fastq file")]
    NotAFastqFile,

    /// File seems not containts fasta data
    #[error("Input file seems not be a fasta file")]
    NotAFastaFile,

    /// File seems not containts vcf data
    #[error("Input file seems not be a vcf file")]
    NotAVcfFile,

    /// Vcf error
    #[error(transparent)]
    VcfError(#[from] VcfError),

    /// Current record seems to be a partial record
    #[error("biommap found a partial record")]
    PartialRecord,
}

/// Enum to manage vcf error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum VcfError {
    /// Blocksize of header reader isn't larger than header increased it
    #[error("Blocksize of header reader isn't larger than header increased it")]
    HeaderBlockTooShort,

    /// Header Vcf Info header not complete
    #[error("A header info record not containt ID, Number, Type and Description")]
    HeaderInfoPartial,

    /// Header Vcf Filter header not complete
    #[error("A header filter record not containt ID, Number, Type and Description")]
    HeaderFilterPartial,

    /// Header Vcf Format header not complete
    #[error("A header format record not containt ID, Number, Type and Description")]
    HeaderFormatPartial,
}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
