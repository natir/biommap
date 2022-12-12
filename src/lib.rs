//! Ar efficient bioinformatics file parser based on memory mapping of file.

#![warn(missing_docs)]

/* std use */

/* crate use */

/* project use */

/* mod declaration */
pub mod block;
pub mod error;

#[cfg(feature = "fasta")]
pub mod fasta;
#[cfg(feature = "fastq")]
pub mod fastq;
#[cfg(feature = "vcf")]
pub mod vcf;

pub mod parser;

/// Define default blocksize
pub const DEFAULT_BLOCKSIZE: u64 = 65536;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::SeedableRng;
    use std::io::Write;

    pub fn generate_fastq(seed: u64, nb_seq: usize, length: usize) -> tempfile::NamedTempFile {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let dna = [b'A', b'C', b'T', b'G'];
        let qual = (0..94).collect::<Vec<u8>>();

        for i in 0..nb_seq {
            let dna_seq = (0..length)
                .map(|_| dna[rng.gen_range(0..4)] as char)
                .collect::<String>();
            let qual_seq = (0..length)
                .map(|_| (qual[rng.gen_range(0..94)] + 33) as char)
                .collect::<String>();

            writeln!(file, "@{}\n{}\n+{}\n{}", i, dna_seq, i, qual_seq).unwrap();
        }

        file
    }

    #[allow(dead_code)]
    pub fn generate_fasta(seed: u64, nb_seq: usize, length: usize) -> tempfile::NamedTempFile {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let dna = [b'A', b'C', b'T', b'G'];

        for i in 0..nb_seq {
            let dna_seq = (0..length)
                .map(|_| dna[rng.gen_range(0..4)] as char)
                .collect::<String>();

            writeln!(file, ">{}\n{}", i, dna_seq).unwrap();
        }

        file
    }
}
