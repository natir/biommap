//! Ar efficient bioinformatics file parser based on memory mapping of file.

#![warn(missing_docs)]

/* std use */

/* crate use */

use clap::Parser as _;

/* project use */

/// Enum to select sequence type
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum SequenceType {
    /// Fasta sequence type
    #[default]
    Fasta,

    /// Fastq sequence type
    Fastq,
}

impl std::fmt::Display for SequenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SequenceType::Fasta => write!(f, "fasta"),
            SequenceType::Fastq => write!(f, "fastq"),
        }
    }
}

#[biommap::derive::sequential_parser(name = CountFastaRecord, data_type = u64, block_type = biommap::block::Block<memmap2::Mmap>, block_producer = biommap::fasta::File2Block, record_producer = biommap::fasta::Block2Record)]
fn parser(&mut self, _record: biommap::fasta::Record, counter: &mut u64) {
    *counter += 1;
}

#[biommap::derive::sequential_parser(name = CountFastqRecord, data_type = u64, block_type = biommap::block::Block<memmap2::Mmap>, block_producer = biommap::fastq::File2Block, record_producer = biommap::fastq::Block2Record)]
fn parser(&mut self, _record: biommap::fastq::Record, counter: &mut u64) {
    *counter += 1;
}

/// Example: Count fasta record in file.
#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "biommap",
    version = "0.1",
    author = "Pierre Marijon <pierre@marijon.fr>"
)]
pub struct Command {
    /// Input path
    #[clap(short = 'i', long = "input")]
    pub input_path: std::path::PathBuf,

    /// Block size
    #[clap(short = 'b', long = "block-size")]
    pub block_size: u64,

    /// Sequence type
    #[clap(short = 't', long = "type", value_enum)]
    pub sequence_type: SequenceType,

    // Basic parameter
    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    pub ts: Option<stderrlog::Timestamp>,
}

fn main() -> anyhow::Result<()> {
    // parse cli
    let params = Command::parse();

    // Setup logger
    stderrlog::new()
        .quiet(params.quiet)
        .verbosity(params.verbosity as usize)
        .timestamp(params.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();

    let mut records_counter = 0;

    match params.sequence_type {
        SequenceType::Fasta => {
            log::info!("fasta mode");

            let mut parser = CountFastaRecord::new();

            parser.with_blocksize(params.block_size, &params.input_path, &mut records_counter)?;
        }
        SequenceType::Fastq => {
            log::info!("fastq mode");

            let mut parser = CountFastqRecord::new();

            parser.with_blocksize(params.block_size, &params.input_path, &mut records_counter)?;
        }
    }

    println!(
        "{} contains {} {} records",
        params.input_path.into_os_string().into_string().unwrap(),
        records_counter,
        params.sequence_type,
    );

    Ok(())
}
