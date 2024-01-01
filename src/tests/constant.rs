//! Define constant use for test and benchmarking

/* std use */

/* crate use */

/* project use */

pub const SEED: [u8; 32] = [42; 32];

pub const SEQUENCE_ALPHABET: &[u8] = b"ACTGactg";
pub const QUALITY_ALPHABET: &[u8] = b"!#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHI"; // Phred + 33 score with " because
