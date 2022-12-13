//! Module provide a macro to run a sequential parsing of file
//!
//! Also contains macro to easily build fasta and fastq parser

/// Macro to generate a sequential parser
#[macro_export(local_inner_macros)]
macro_rules! impl_sequential {
    ($name:ident, $producer:expr, $reader:expr,  $data_type:ty, $record:expr, $record_type:ty,) => {
        pub struct $name {}

        impl $name {
            pub fn new() -> Self {
                Self {}
            }

            pub fn parse<P>(&mut self, path: P, data: &mut $data_type) -> $crate::error::Result<()>
            where
                P: AsRef<std::path::Path>,
            {
                self.with_blocksize($crate::DEFAULT_BLOCKSIZE, path, data)
            }

            pub fn with_blocksize<P>(
                &mut self,
                blocksize: u64,
                path: P,
                data: &mut $data_type,
            ) -> $crate::error::Result<()>
            where
                P: AsRef<std::path::Path>,
            {
                let mut producer = $producer(blocksize, path)?;

                while let Some(block) = producer.next_block()? {
                    self.block(block, data)?
                }

                Ok(())
            }

            fn block(
                &mut self,
                block: $crate::block::Block,
                data: &mut $data_type,
            ) -> $crate::error::Result<()> {
                let mut reader = $reader(block);

                while let Some(record) = reader.next_record()? {
                    self.record(record, data);
                }

                Ok(())
            }

            fn record(&self, record: $record_type, data: &mut $data_type) -> () {
                $record(record, data);
            }
        }
    };
}

#[cfg(feature = "fasta")]
/// Macro to generate a fasta sequential parser
#[macro_export(local_inner_macros)]
macro_rules! fasta_sequential {
    ($name:ident, $data_type:ty, $record:expr) => {
        impl_sequential!(
            $name,
            $crate::fasta::Producer::with_blocksize,
            $crate::fasta::Reader::new,
            $data_type,
            $record,
            $crate::fasta::Record,
        );
    };
}

/// Macro to generate a fasta sequential parser/// Macro to generate a fasta sequential parser
#[cfg(feature = "fastq")]
#[macro_export(local_inner_macros)]
macro_rules! fastq_sequential {
    ($name:ident, $data_type:ty, $record:expr) => {
        impl_sequential!(
            $name,
            $crate::fastq::Producer::with_blocksize,
            $crate::fastq::Reader::new,
            $data_type,
            $record,
            $crate::fastq::Record,
        );
    };
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "fasta")]
    use crate::fasta;
    #[cfg(feature = "fastq")]
    use crate::fastq;

    #[cfg(feature = "fasta")]
    #[test]
    fn record_count_fasta() -> error::Result<()> {
        impl_sequential!(
            FastaRecordCount,
            fasta::Producer::with_blocksize,
            fasta::Reader::new,
            u64,
            |_record: fasta::Record, counter: &mut u64| {
                *counter += 1;
            },
            fasta::Record,
        );

        let mut counter = 0;

        let mut parser = FastaRecordCount::new();

        parser.parse(crate::tests::generate_fasta(42, 1_000, 150)?, &mut counter)?;

        assert_eq!(1_000, counter);

        Ok(())
    }

    #[cfg(feature = "fasta")]
    #[test]
    fn base_count_fasta() -> error::Result<()> {
        impl_sequential!(
            FastaNucCount,
            fasta::Producer::with_blocksize,
            fasta::Reader::new,
            [u64; 4],
            |record: fasta::Record, bases: &mut [u64; 4]| {
                for nuc in record.sequence {
                    bases[(nuc >> 1 & 0b11) as usize] += 1;
                }
            },
            fasta::Record,
        );

        let mut bases = [0; 4];

        let mut parser = FastaNucCount::new();

        parser.parse(crate::tests::generate_fasta(42, 1_000, 150)?, &mut bases)?;

        assert_eq!([37378, 37548, 37548, 37526], bases);

        Ok(())
    }

    #[cfg(feature = "fastq")]
    #[test]
    fn record_count_fastq() -> error::Result<()> {
        impl_sequential!(
            FastqRecordCount,
            fastq::Producer::with_blocksize,
            fastq::Reader::new,
            u64,
            |_record: fastq::Record, counter: &mut u64| {
                *counter += 1;
            },
            fastq::Record,
        );

        let mut counter = 0;

        let mut parser = FastqRecordCount::new();

        parser
            .parse(crate::tests::generate_fastq(42, 1_000, 150)?, &mut counter)
            .unwrap();

        assert_eq!(1_000, counter);

        Ok(())
    }

    #[cfg(feature = "fastq")]
    #[test]
    fn base_count_fastq() -> error::Result<()> {
        impl_sequential!(
            FastqNucCount,
            fastq::Producer::with_blocksize,
            fastq::Reader::new,
            [u64; 4],
            |record: fastq::Record, bases: &mut [u64; 4]| {
                for nuc in record.sequence {
                    bases[(nuc >> 1 & 0b11) as usize] += 1;
                }
            },
            fastq::Record,
        );

        let mut bases = [0; 4];

        let mut parser = FastqNucCount::new();

        parser.parse(crate::tests::generate_fastq(42, 1_000, 150)?, &mut bases)?;

        assert_eq!([37301, 37496, 37624, 37579], bases);

        Ok(())
    }
}
