//! Map reduce

/* std use */

/* crate use */

/* project use */

/// Trait required for map reduce
pub trait Accumulator: std::marker::Sized + std::default::Default {
    /// Accumulate function
    fn accumulate(&mut self, other: Self);
}

#[cfg(feature = "parallel")]
mod tmp {
    use crate as biommap;
    use crate::parser::map_reduce::Accumulator;
    use rayon::prelude::*;

    #[derive(Default)]
    struct SAccumulator {
        pub inner: [u64; 4],
    }

    impl crate::parser::map_reduce::Accumulator for SAccumulator {
        fn accumulate(&mut self, other: Self) {
            for i in 0..4 {
                self.inner[i] += other.inner[i];
            }
        }
    }

    #[crate::derive::map_reduce_parser(name = CountNuc, data_type = SAccumulator, block_producer = crate::fasta::File2Block, record_producer = crate::fasta::Block2Record)]
    fn parser(record: crate::fasta::Record) -> SAccumulator {
        let data = SAccumulator::default();

        for nuc in record.sequence() {
            data.inner[((nuc >> 1) & 0b11) as usize] += 1;
        }

        data
    }
}
