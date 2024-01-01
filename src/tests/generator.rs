//! Define function to generate sequence value use for test and benchmarking

/* std use */

/* crate use */
use rand::prelude::*;

#[cfg(parallel)]
use rayon::slice::ParallelSliceMut;

/* project use */
use super::constant;

/// Generate a RNG with constant::SEED
pub fn rng() -> rand::rngs::StdRng {
    rand::rngs::StdRng::from_seed(constant::SEED)
}

/// Generate a random DNA sequence with constant::SEQUENCE_ALPHABET
pub fn seq(rng: &mut rand::rngs::StdRng, seq_length: u64) -> Vec<u8> {
    (0..seq_length)
        .map(|_| *constant::SEQUENCE_ALPHABET.choose(rng).unwrap())
        .collect::<Vec<u8>>()
}

/// Generate a random DNA sequence with constant::QUALITY_ALPHABET
pub fn quality(rng: &mut rand::rngs::StdRng, seq_length: u64) -> Vec<u8> {
    (0..seq_length)
        .map(|_| *constant::QUALITY_ALPHABET.choose(rng).unwrap())
        .collect::<Vec<u8>>()
}

/// Generate a random in ram fasta with RNG
pub fn fasta(rng: &mut rand::rngs::StdRng, seq_length: u64, seq_number: u64) -> Vec<u8> {
    let mut output = Vec::with_capacity(
        (
            seq_length * seq_number // sequence space
		+ seq_number * 3 // '>' and jump line space
		+ (seq_number.checked_ilog10().unwrap_or(0) as u64 + 1) * seq_number
            // sequence id space
        ) as usize,
    );

    for index in 0..seq_number {
        // Header
        output.extend(b">");
        output.extend(index.to_string().as_bytes());
        output.extend(b"\n");
        // Sequence
        output.extend(seq(rng, seq_length));
        output.extend(b"\n");
    }

    output
}

/// Generate a random in ram fastq with RNG
pub fn fastq(rng: &mut rand::rngs::StdRng, seq_length: u64, seq_number: u64) -> Vec<u8> {
    let mut output = Vec::with_capacity(
        (
            seq_length * seq_number // sequence space
            + seq_length * seq_number // quality space
            + seq_number * 6 // '@' '+' and jump line space
	+ (seq_number.checked_ilog10().unwrap_or(0) as u64 + 1) * seq_number
            // sequence id space
        ) as usize,
    );

    for index in 0..seq_number {
        // Header
        output.extend(b"@");
        output.extend(index.to_string().as_bytes());
        output.extend(b"\n");
        // Sequence
        output.extend(seq(rng, seq_length));
        output.extend(b"\n");
        // Plus
        output.extend(b"+\n");
        // Quality
        output.extend(quality(rng, seq_length));
        output.extend(b"\n");
    }

    output
}
