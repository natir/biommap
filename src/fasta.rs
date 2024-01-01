//! Fasta parsing

/* std use */

/* crate use */
use bstr::ByteSlice;

/* project use */
use crate::block;
use crate::error;

#[cfg(feature = "derive")]
#[biommap_derive::file2block(name = File2Block, block_type = memmap2::Mmap)]
fn fasta(block: &[u8]) -> error::Result<u64> {
    let mut end = block.len();

    for _ in 0..2 {
        end = block[..end]
            .rfind_byte(b'\n')
            .ok_or(error::Error::NoNewLineInBlock)?;

        if end + 1 < block.len() && block[end + 1] == b'>' {
            return Ok((end + 1) as u64);
        }
    }

    Err(error::Error::NotAFastaFile)
}

/// Struct that store a fasta record
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Record<'a> {
    comment: &'a [u8],
    sequence: &'a [u8],
}

impl<'a> Record<'a> {
    /// Fasta comment without `>`
    pub fn comment(&self) -> &'a [u8] {
        &self.comment
    }

    /// Fasta sequence
    pub fn sequence(&self) -> &'a [u8] {
        &self.sequence
    }
}

#[cfg(feature = "derive")]
#[biommap_derive::block2record(name = Block2Record, generic_type = DATA)]
pub fn fasta(&mut self) -> error::Result<Option<Record<'_>>> {
    if self.offset == self.block.len() {
        Ok(None)
    } else {
        let comment = &self.block.data()[self.get_line()?];
        self.offset += comment.len() as u64 + 1;

        let sequence = &self.block.data()[self.get_line()?];
        self.offset += sequence.len() as u64 + 1;

        Ok(Some(Record { comment, sequence }))
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* project use */
    use crate::error;

    /* local use */
    use super::*;

    #[cfg(feature = "derive")]
    #[test]
    fn default() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fasta(&mut rng, 150, 100, temp.path())?;

        let mut producer = File2Block::new(temp.path())?;

        let option = producer.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 8050);

        let option = producer.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 7440);

        let option = producer.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn blocksize() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fasta(&mut rng, 150, 100, temp.path())?;

        let mut producer = File2Block::with_blocksize(8192 * 2, temp.path())?;

        let option = producer.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 15490);

        let option = producer.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn offset() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fasta(&mut rng, 150, 100, temp.path())?;

        let mut producer = File2Block::with_offset(8050, temp.path())?;

        let option = producer.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 7440);

        let option = producer.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn blocksize_offset() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fasta(&mut rng, 150, 100, temp.path())?;

        let mut producer = File2Block::with_blocksize_offset(4096, 8050, temp.path())?;

        let option = producer.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 4030);

        let option = producer.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 3410);

        let option = producer.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn records() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fasta(&mut rng, 50, 10, temp.path())?;

        let mut comments = Vec::new();
        let mut seqs = Vec::new();

        let mut producer = File2Block::new(temp.path())?;

        while let Ok(Some(block)) = producer.next_block() {
            let mut reader = Block2Record::new(block);

            while let Ok(Some(record)) = reader.next_record() {
                comments.push(String::from_utf8(record.comment().to_vec()).unwrap());
                seqs.push(String::from_utf8(record.sequence().to_vec()).unwrap());
            }
        }

        assert_eq!(
            comments,
            vec![
                ">0".to_string(),
                ">1".to_string(),
                ">2".to_string(),
                ">3".to_string(),
                ">4".to_string(),
                ">5".to_string(),
                ">6".to_string(),
                ">7".to_string(),
                ">8".to_string(),
                ">9".to_string()
            ]
        );
        assert_eq!(
            seqs,
            vec![
                "taTATgAAtCGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAta".to_string(),
                "TcgAAtTaTaGaTggttGCtCatGtctgCTGGTACtgTgcaaaagggGAG".to_string(),
                "acAtgCtGCAAtTacCGtTAAcaGGtatTCaTCctcTGgAActTgCGAca".to_string(),
                "AgaAAtaTCCcAgagggaCcttCcGcTTGcgAACcTtCttAacGtTtAtG".to_string(),
                "TgACAGCCaCGctGagattTGtgCttaAGggTcCTGcGTAGCTGTCCACg".to_string(),
                "TTTGagtGaGCatAGGACAAaacTaTTagagGtatAGCcTatTtaaaaCG".to_string(),
                "gcttGGTtgaCtgACTacgtCTaTgTCAGgCtaGTtcCCTcgcTgAgGgA".to_string(),
                "tCAAatTCTATTGTaggcGCaCcCGtCtATgTTgTATcaTTCGaCCttcA".to_string(),
                "aGCGCAatgaTGAtaatcaCtGcTAGCCAgaTTgcAaTtaTGgACTTagG".to_string(),
                "gtATACCtcTctCAtgCGCagTCTcaacCATAtGtGgtAtacAagtTGgA".to_string()
            ]
        );

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn not_fasta() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fastq(&mut rng, 50, 2, temp.path())?;

        let mut producer = File2Block::with_blocksize(100, temp.path())?;

        let result = producer.next_block();
        assert_matches::assert_matches!(result, Err(error::Error::NotAFastaFile));

        Ok(())
    }
}
