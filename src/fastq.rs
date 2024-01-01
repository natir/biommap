//! Fastq parsing

/* std use */

/* crate use */
use bstr::ByteSlice;

/* project use */
use crate::block;
use crate::error;

#[cfg(feature = "derive")]
#[biommap_derive::file2block(name = File2Block, block_type = memmap2::Mmap)]
fn fastq(block: &[u8]) -> error::Result<u64> {
    let mut end = block.len();

    for _ in 0..5 {
        end = block[..end]
            .rfind_byte(b'\n')
            .ok_or(error::Error::NoNewLineInBlock)?;

        if end + 1 < block.len() && block[end + 1] == b'@' {
            let prev = block[..end]
                .rfind_byte(b'\n')
                .ok_or(error::Error::NoNewLineInBlock)?;
            if block[prev + 1] == b'+' {
                let prevprev = block[..prev]
                    .rfind_byte(b'\n')
                    .ok_or(error::Error::NoNewLineInBlock)?;
                if block[prevprev + 1] == b'+' {
                    return Ok((end + 1) as u64);
                } else {
                    let prevprevprev = block[..prevprev]
                        .rfind_byte(b'\n')
                        .ok_or(error::Error::NoNewLineInBlock)?;
                    if block[prevprevprev + 1] == b'@' {
                        return Ok((prevprevprev + 1) as u64);
                    } else {
                        return Err(error::Error::NotAFastqFile);
                    }
                }
            } else {
                return Ok((end + 1) as u64);
            }
        }
    }

    Err(error::Error::NotAFastqFile)
}

/// Struct that store a fastq record
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Record<'a> {
    comment: &'a [u8],
    sequence: &'a [u8],
    plus: &'a [u8],
    quality: &'a [u8],
}

impl<'a> Record<'a> {
    /// Fastq comment, without `>`
    pub fn comment(&self) -> &'a [u8] {
        &self.comment
    }

    /// Fastq sequence
    pub fn sequence(&self) -> &'a [u8] {
        &self.sequence
    }

    /// Fastq plus line, without `+`
    pub fn plus(&self) -> &'a [u8] {
        &self.plus
    }

    /// Fastq quality
    pub fn quality(&self) -> &'a [u8] {
        &self.quality
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

        let plus = &self.block.data()[self.get_line()?];
        self.offset += plus.len() as u64 + 1;

        let quality = &self.block.data()[self.get_line()?];
        self.offset += quality.len() as u64 + 1;

        Ok(Some(Record {
            comment,
            sequence,
            plus,
            quality,
        }))
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* project use */

    /* local use */
    use super::*;

    #[cfg(feature = "derive")]
    #[test]
    fn default() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fastq(&mut rng, 150, 100, temp.path())?;

        let mut blocks = File2Block::new(temp.path())?;

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 7998);

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 8008);

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 8008);

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 6776);

        let option = blocks.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn blocksize() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fastq(&mut rng, 150, 100, temp.path())?;

        let mut blocks = File2Block::with_blocksize(8192 * 2, temp.path())?;

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 16314);

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 14476);

        let option = blocks.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn offset() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fastq(&mut rng, 150, 100, temp.path())?;

        let mut blocks = File2Block::with_offset(24014, temp.path())?;

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 6776);

        let option = blocks.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn blocksize_offset() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fastq(&mut rng, 150, 100, temp.path())?;

        let mut blocks = File2Block::with_blocksize_offset(4096, 24014, temp.path())?;

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 4004);

        let option = blocks.next_block()?;
        assert!(option.is_some());
        let block = option.unwrap();
        assert_eq!(block.len(), 2772);

        let option = blocks.next_block()?;
        assert!(option.is_none());

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn records() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;
        let mut rng = crate::tests::generator::rng();

        crate::tests::io::write_fastq(&mut rng, 50, 10, temp.path())?;

        let mut comments = Vec::new();
        let mut seqs = Vec::new();
        let mut pluss = Vec::new();
        let mut quals = Vec::new();

        let mut blocks = File2Block::new(temp.path())?;

        while let Ok(Some(block)) = blocks.next_block() {
            let mut reader = Block2Record::new(block);

            while let Ok(Some(record)) = reader.next_record() {
                comments.push(String::from_utf8(record.comment().to_vec()).unwrap());
                seqs.push(String::from_utf8(record.sequence().to_vec()).unwrap());
                pluss.push(String::from_utf8(record.plus().to_vec()).unwrap());
                quals.push(String::from_utf8(record.quality().to_vec()).unwrap());
            }
        }

        assert_eq!(
            comments,
            vec![
                "@0".to_string(),
                "@1".to_string(),
                "@2".to_string(),
                "@3".to_string(),
                "@4".to_string(),
                "@5".to_string(),
                "@6".to_string(),
                "@7".to_string(),
                "@8".to_string(),
                "@9".to_string()
            ]
        );
        assert_eq!(
            seqs,
            vec![
                "taTATgAAtCGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAta".to_string(),
                "agggGAGacAtgCtGCAAtTacCGtTAAcaGGtatTCaTCctcTGgAAct".to_string(),
                "GtTtAtGTgACAGCCaCGctGagattTGtgCttaAGggTcCTGcGTAGCT".to_string(),
                "AGCcTatTtaaaaCGgcttGGTtgaCtgACTacgtCTaTgTCAGgCtaGT".to_string(),
                "CtATgTTgTATcaTTCGaCCttcAaGCGCAatgaTGAtaatcaCtGcTAG".to_string(),
                "tgCGCagTCTcaacCATAtGtGgtAtacAagtTGgAtgcGtTCtctTgct".to_string(),
                "tgCaaatgctgTcCaAgttcGtGAtcAttaTtGgCACgCcgcCgATtcGC".to_string(),
                "gAcCGgACTctgTGTtaAGCAgcagAcGttCagTgCTAtccTGAAccCaa".to_string(),
                "ttCGTTaGccGaCAaGCGGATCgGGGATCaAaGcaACCGaTcGGCCGgGa".to_string(),
                "tAGCCtCTgATTtTGCcGcGgCgTcGcTatcaaaACTaaGATtaaGaAcg".to_string(),
            ]
        );
        assert_eq!(
            pluss,
            vec![
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
                "+".to_string(),
            ]
        );
        assert_eq!(
            quals,
            vec![
                "=EC!:3@9D-41D6.E/%/>BA30'>(7B1B%AE'5-2,>!0(AFE;D68".to_string(),
                "G&&'2<CH8#GH?!%!4B7,:$)8F8=@@D<+295-<F?.#>CI4@#7<&".to_string(),
                "1-(!'F--C-I3C7EA3?72.C(!12#(!I#;->%+%7+.:6GI6E3@CB".to_string(),
                "B(I+;=,/+#G%1)E0#A(D*#I6B.(-5$-.I.I07EIGC<(/=='1B>".to_string(),
                ")/)C#0F+I-&$G&4#%D@+=-C*F#,-*0G1FA5?I()@9:&,=A/(0!".to_string(),
                "@1F5>-9BH=9F?+*>38->/G/E@(,#*>B82$0/FG:/$#DI240.G=".to_string(),
                "%6,8$-125'B8,7:G/?;?C$H'2AB%-0'B*4A#',?*=%AA$0:C#D".to_string(),
                "!</6DI7G*'&#.'%-I6.G?:<F718>8C#47/(36*D5,BIHD4++F9".to_string(),
                "<#0/,00<?8<>E5.3#/EB&,'B.$%6G?E*)--H@;;(<#CG:8FB09".to_string(),
                "73I55(+37.>0;&FF1474%;:/81>59@9>%(H0'8;95!<)8(;*;%".to_string(),
            ]
        );

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn quality_is_shit() -> error::Result<()> {
        let data = b"@1\nAA\n+1\n!!\n@2\nTT\n+2\n!!";
        assert_eq!(File2Block::correct_block_size(data)?, 12);

        let data = b"@1\nAA\n+1\n!!\n@2\nTT\n+2\n+!\n@3";
        assert_eq!(File2Block::correct_block_size(data)?, 24);

        let data = b"@1\nAA\n+1\n!!\n@2\nTT\n+2\n@!";
        assert_eq!(File2Block::correct_block_size(data)?, 12);

        Ok(())
    }

    #[cfg(feature = "derive")]
    #[test]
    fn not_a_fastq() -> error::Result<()> {
        let temp = tempfile::NamedTempFile::new()?;

        let mut data = b"@0
TTAGATTATAGTACGG
ATTATAT
+1
AGTTATCGTGTACCTC
+1
+CW?:KL~15\\E|MN
GTCCCTCAATCCG
+2
"
        .to_vec();

        crate::tests::io::write_buffer(&data, temp.path())?;

        let mut blocks = File2Block::with_blocksize(82, temp.path())?;

        assert!(blocks.next_block().is_err());

        data.extend(
            b"+FAILLED FILE
+3
+TTGGGCATGAGGTTCA
@3ueauie
+~vGLKg+n!*iJ\\K
@iuiea
",
        );
        crate::tests::io::write_buffer(&data, temp.path())?;

        let mut blocks = File2Block::with_blocksize(82, temp.path())?;

        assert!(blocks.next_block().is_err());

        let mut blocks = File2Block::with_blocksize(82, temp.path())?;
        assert!(blocks.next().is_some());
        assert!(blocks.next().unwrap().is_err());

        Ok(())
    }
}
