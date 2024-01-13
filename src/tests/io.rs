//! Define function to write or read information

/* std use */

use std::io::Write as _;

/* crate use */

/* project use */
use super::generator;

pub fn write_buffer<P>(buffer: &[u8], path: P) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let mut file = std::io::BufWriter::new(std::fs::File::create(&path)?);

    file.write_all(buffer)?;

    Ok(())
}

#[allow(dead_code)]
/// Write a random fasta in path
pub fn write_fasta<P>(
    rng: &mut rand::rngs::StdRng,
    seq_length: u64,
    seq_number: u64,
    path: P,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    write_buffer(&generator::fasta(rng, seq_length, seq_number), path)
}

#[allow(dead_code)]
/// Write a random fastq in path
pub fn write_fastq<P>(
    rng: &mut rand::rngs::StdRng,
    seq_length: u64,
    seq_number: u64,
    path: P,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    write_buffer(&generator::fastq(rng, seq_length, seq_number), path)
}
