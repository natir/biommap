//! Struct that extract part of file (called block) and read it as fastq file.

/* std use */

/* crate use */
use bstr::ByteSlice;

/* project use */
use crate::block;
use crate::error;
use crate::impl_producer;
use crate::impl_reader;

/// Strutt that store a fastq record
pub struct Record<'a> {
    /// Fastq comment, without `>`
    pub comment: &'a [u8],
    /// Fastq sequence
    pub sequence: &'a [u8],
    /// Fastq plus line, without `>`
    pub plus: &'a [u8],
    /// Fastq quality
    pub quality: &'a [u8],
}

impl_producer!(Producer, |block: &[u8]| {
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
});

impl_reader!(
    Reader,
    'a,
    Record,
    |block: &'a block::Block, offset: &mut usize| {
        if *offset == block.len() {
            Ok(None)
        } else {
            let comment = &block.data()[Self::get_line(block, offset)?];
            *offset += comment.len() + 1;

            let sequence = &block.data()[Self::get_line(block, offset)?];
            *offset += sequence.len() + 1;

            let plus = &block.data()[Self::get_line(block, offset)?];
            *offset += plus.len() + 1;

            let quality = &block.data()[Self::get_line(block, offset)?];
            *offset += quality.len() + 1;

            Ok(Some(Record {
                comment,
                sequence,
                plus,
                quality,
            }))
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    mod producer {
        use super::*;

        use std::io::Write;

        #[test]
        fn new() -> error::Result<()> {
            let file = crate::tests::generate_fastq(42, 100, 150)?;
            let mut tmp = Producer::new(file).unwrap();

            let block = tmp.next_block()?.unwrap();

            assert_eq!(block.len(), 30980);

            assert!(tmp.next_block()?.is_none());

            Ok(())
        }

        #[test]
        fn with_blocksize() -> error::Result<()> {
            let file = crate::tests::generate_fastq(42, 100, 150)?;
            let mut tmp = Producer::with_blocksize(463, file).unwrap();

            let block = tmp.next_block()?.unwrap();

            assert_eq!(block.len(), 308);

            assert_eq!(
                String::from_utf8(block.data().to_vec()).unwrap(),
                "@0
TTAGATTATAGTACGGTATAGTGGTTACTATGTAGCCTAAGTGGCGCCCGTTGTAGAGGAATCCACTTATATAACACAGGTATAATCCGGACGGCATGCGCAGGCATGCCTATATTCTATGACAGCAGGATTATGGAAGATGGTGCTCTA
+0
^U3<L0PV{cnrl:8N`!:=mF8M0!0Ez/d{4j$=9f5rLeAQ-H.ptT3w6aMy8Z6O-dZ}2`UX=YJ-Etg`s&B%~F!kR7S8]@lTI<2-\\';v0}hU.(T*0VHGx,>Gze)*5rFv}k@RllOE2K)\"DQJvO)bl?(dDhh
".to_string()
            );

            Ok(())
        }

        #[test]
        fn with_blocksize_offset() -> error::Result<()> {
            let file = crate::tests::generate_fastq(42, 100, 150)?;
            let mut tmp = Producer::with_blocksize_offset(463, 308, file).unwrap();

            let block = tmp.next_block()?.unwrap();

            assert_eq!(block.len(), 308);

            assert_eq!(
                String::from_utf8(block.data().to_vec()).unwrap(),
                "@1
AGTTATCGTGTACCTCCTAGCTTTTAGTTGTGCTTTAACAGTGTAACATTGGGACGCTATTACTCGCCGGTGAGGCGGTCTTCCTTGACTATACCGATCGTGGAGTTCATGCGCGCGGATCCCTCAGCGTTCTCGGGAAGCGCGAACAGA
+1
iCW?:KL~15\\E|MNRKY)S$?~~Ub}d)dY2LX:e@b^'<<$$e56W0fdV,<Y>Yd(J<5p6xt)z+OxuPXv?/_yH8z^%Sks1*nxm$<7*YdkvNPf:>YW=$uxZ)}[v/DlZm&EW(s(cMelx\"iEV3Hp]cz3%_T@\\Ms
".to_string()
            );

            Ok(())
        }

        #[test]
        fn with_blocksize_buffer_larger_file() -> error::Result<()> {
            let file = crate::tests::generate_fastq(42, 100, 150)?;
            let mut tmp = Producer::with_blocksize(8092, file).unwrap();

            let block = tmp.next_block().unwrap().unwrap();

            assert_eq!(block.len(), 616);

            Ok(())
        }

        #[test]
        fn get_all_block() -> error::Result<()> {
            let file = crate::tests::generate_fastq(42, 100, 150)?;
            let mut tmp = Producer::new(file).unwrap();

            let mut block_length = Vec::new();
            while let Ok(Some(block)) = tmp.next_block() {
                block_length.push(block.len());
            }

            assert_eq!(block_length, vec![65300, 65520, 65520, 65520, 49920]);

            Ok(())
        }

        #[test]
        fn check_block() -> error::Result<()> {
            let file = crate::tests::generate_fastq(42, 100, 150)?;

            let mut tmp = Producer::with_blocksize(800, file)?;

            assert_eq!(
                String::from_utf8(tmp.next_block()?.unwrap().data().to_vec()),
                Ok("@0
TTAGATTATAGTACGGTATAGTGGTTACTATGTAGCCTAAGTGGCGCCCGTTGTAGAGGAATCCACTTATATAACACAGGTATAATCCGGACGGCATGCGCAGGCATGCCTATATTCTATGACAGCAGGATTATGGAAGATGGTGCTCTA
+0
^U3<L0PV{cnrl:8N`!:=mF8M0!0Ez/d{4j$=9f5rLeAQ-H.ptT3w6aMy8Z6O-dZ}2`UX=YJ-Etg`s&B%~F!kR7S8]@lTI<2-\\';v0}hU.(T*0VHGx,>Gze)*5rFv}k@RllOE2K)\"DQJvO)bl?(dDhh
@1
AGTTATCGTGTACCTCCTAGCTTTTAGTTGTGCTTTAACAGTGTAACATTGGGACGCTATTACTCGCCGGTGAGGCGGTCTTCCTTGACTATACCGATCGTGGAGTTCATGCGCGCGGATCCCTCAGCGTTCTCGGGAAGCGCGAACAGA
+1
iCW?:KL~15\\E|MNRKY)S$?~~Ub}d)dY2LX:e@b^'<<$$e56W0fdV,<Y>Yd(J<5p6xt)z+OxuPXv?/_yH8z^%Sks1*nxm$<7*YdkvNPf:>YW=$uxZ)}[v/DlZm&EW(s(cMelx\"iEV3Hp]cz3%_T@\\Ms
".to_string())
            );
            assert_eq!(
                String::from_utf8(tmp.next_block()?.unwrap().data().to_vec()),
                Ok("@2
AATGTCCCTCAATCCGCGGCATGGCTAAGTACCACCGTGGATGTAAATTTTTCAGTCGTCTCTTCATACTGTTCCTGTACTGTCAGGGATGCTCCCTTTCACAGAGCTCGTATAATCAGTAAACGCCACGGTCCTTTCTCTGTTAACCGC
+2
Ouf)Y|l;S1!tk[U9n2(NK=#Tmg,t+CSsUMaPs7{+V'~On{hc1NR}aY^YbYlg[}Fcq1K_$v1HG\"tRBj`||g>\\)2pU_QrnWO{c@;lw8B0+urH~$#K>:iSa-I-C#gDJ(9UUFubOeRHsDX3Ko`?T--iL+j
@3
TTGGGCATGAGGTTCACCGAAGGTGGCAGATATGCGCCATAAATTGACCAGGTTGTATCCAGCATTGGAAGAACGCACCCGGGGGGAGCACAGATCCTAGCAGTACACGCTCTGGGTCCTCTACGTCTTCGGAGTCTCTAGCTTGCCTTA
+3
:~vGLKg+n!*iJ\\.*wfxK)5Qmh%<:f^$nql7OB$}M/d.F,5[=>ZW*#f=0>Ao(@~lEHbSG1%,b_Uy2!zL%2GMB0O.t[#UcQ[]ufFZJ!K<kLgDNQlx)s8+75E^[-\"!1l[i<S#G\"B]xZ5?as*@8Laq`{@r
".to_string())
            );
            assert_eq!(
                String::from_utf8(tmp.next_block()?.unwrap().data().to_vec()),
                Ok("@4
TCTATAGCTTGTCATGCCTTTCGATTGAGGGCGTCACCAAGCGAATTACTCGCTGATCCGTTCCCCGCCAATTCTGAGACTCCATAATCCTATCTGTGTCCCTAGGTGCCGTGTTCCGGTCGTGAGTTCGGCCCTTGCCTAAAGTTAATG
+4
myS=C|jEWnl,aC\\7!jv9[!vh/PAK}_H&<.o]qf|y@4L:?ssLg3N!v7/N5RyPHn=5%Fyh(4-Z:<6wf]^#t~0:i(X\\l-7]9olH9WLV~`L~JQ7ye7B1RSi2N$PuHwjj\\pb}J\\R~pe?j+X>R#p@MyqBBe*
".to_string())
            );
            assert!(tmp.next_block().is_ok());
            assert!(tmp.next_block().unwrap().is_none());

            Ok(())
        }

        #[test]
        fn quality_is_shit() -> error::Result<()> {
            let data = b"@1\nAA\n+1\n!!\n@2\nTT\n+2\n!!";
            assert_eq!(Producer::correct_block_size(data)?, 12);

            let data = b"@1\nAA\n+1\n!!\n@2\nTT\n+2\n+!\n@3";
            assert_eq!(Producer::correct_block_size(data)?, 24);

            let data = b"@1\nAA\n+1\n!!\n@2\nTT\n+2\n@!";
            assert_eq!(Producer::correct_block_size(data)?, 12);

            Ok(())
        }

        #[test]
        fn not_a_fastq() -> error::Result<()> {
            let mut file = crate::tests::write_in_tempfile(
                b"@0
TTAGATTATAGTACGG
ATTATAT
+1
AGTTATCGTGTACCTC
+1
+CW?:KL~15\\E|MN
GTCCCTCAATCCG
+2
",
            )?;

            let mut producer = Producer::with_blocksize(82, file)?;

            assert!(producer.next_block().is_err());

            {
                let mut rewrite = file.reopen().unwrap();
                rewrite.write(
                    b"+FAILLED FILE
+3
+TTGGGCATGAGGTTCA
@3ueauie
+~vGLKg+n!*iJ\\K
@iuiea
",
                )?;
            }

            let mut producer = Producer::with_blocksize(82, file)?;

            assert!(producer.next_block().is_err());

            let mut producer = Producer::with_blocksize(82, file)?;
            assert!(producer.next().is_some());
            assert!(producer.next().unwrap().is_err());

            Ok(())
        }
    }

    mod reader {
        use super::*;

        #[test]
        fn iterate_over_seq() -> error::Result<()> {
            let mut file = crate::tests::generate_fasta(42, 5, 150)?;
            let mut producer = Producer::with_blocksize(500, file)?;

            let mut comments = Vec::new();
            let mut seqs = Vec::new();
            let mut pluss = Vec::new();
            let mut quals = Vec::new();

            while let Ok(Some(block)) = producer.next_block() {
                let mut reader = Reader::new(block);

                while let Ok(Some(record)) = reader.next_record() {
                    comments.push(String::from_utf8(record.comment.to_vec()).unwrap());
                    seqs.push(String::from_utf8(record.sequence.to_vec()).unwrap());
                    pluss.push(String::from_utf8(record.plus.to_vec()).unwrap());
                    quals.push(String::from_utf8(record.quality.to_vec()).unwrap());
                }
            }

            assert_eq!(
                comments,
                vec![
                    "@0".to_string(),
                    "@1".to_string(),
                    "@2".to_string(),
                    "@3".to_string(),
                    "@4".to_string()
                ]
            );
            assert_eq!(
                seqs,
                vec![
		    "TTAGATTATAGTACGGTATAGTGGTTACTATGTAGCCTAAGTGGCGCCCGTTGTAGAGGAATCCACTTATATAACACAGGTATAATCCGGACGGCATGCGCAGGCATGCCTATATTCTATGACAGCAGGATTATGGAAGATGGTGCTCTA".to_string(),
		    "AGTTATCGTGTACCTCCTAGCTTTTAGTTGTGCTTTAACAGTGTAACATTGGGACGCTATTACTCGCCGGTGAGGCGGTCTTCCTTGACTATACCGATCGTGGAGTTCATGCGCGCGGATCCCTCAGCGTTCTCGGGAAGCGCGAACAGA".to_string(),
		    "AATGTCCCTCAATCCGCGGCATGGCTAAGTACCACCGTGGATGTAAATTTTTCAGTCGTCTCTTCATACTGTTCCTGTACTGTCAGGGATGCTCCCTTTCACAGAGCTCGTATAATCAGTAAACGCCACGGTCCTTTCTCTGTTAACCGC".to_string(), "TTGGGCATGAGGTTCACCGAAGGTGGCAGATATGCGCCATAAATTGACCAGGTTGTATCCAGCATTGGAAGAACGCACCCGGGGGGAGCACAGATCCTAGCAGTACACGCTCTGGGTCCTCTACGTCTTCGGAGTCTCTAGCTTGCCTTA".to_string(), "TCTATAGCTTGTCATGCCTTTCGATTGAGGGCGTCACCAAGCGAATTACTCGCTGATCCGTTCCCCGCCAATTCTGAGACTCCATAATCCTATCTGTGTCCCTAGGTGCCGTGTTCCGGTCGTGAGTTCGGCCCTTGCCTAAAGTTAATG".to_string()     ]
            );
            assert_eq!(
                pluss,
                vec![
                    "+0".to_string(),
                    "+1".to_string(),
                    "+2".to_string(),
                    "+3".to_string(),
                    "+4".to_string()
                ]
            );
            assert_eq!(
                quals,
                vec![
		    "^U3<L0PV{cnrl:8N`!:=mF8M0!0Ez/d{4j$=9f5rLeAQ-H.ptT3w6aMy8Z6O-dZ}2`UX=YJ-Etg`s&B%~F!kR7S8]@lTI<2-\\';v0}hU.(T*0VHGx,>Gze)*5rFv}k@RllOE2K)\"DQJvO)bl?(dDhh".to_string(),
		    "iCW?:KL~15\\E|MNRKY)S$?~~Ub}d)dY2LX:e@b^'<<$$e56W0fdV,<Y>Yd(J<5p6xt)z+OxuPXv?/_yH8z^%Sks1*nxm$<7*YdkvNPf:>YW=$uxZ)}[v/DlZm&EW(s(cMelx\"iEV3Hp]cz3%_T@\\Ms".to_string(),
		    "Ouf)Y|l;S1!tk[U9n2(NK=#Tmg,t+CSsUMaPs7{+V'~On{hc1NR}aY^YbYlg[}Fcq1K_$v1HG\"tRBj`||g>\\)2pU_QrnWO{c@;lw8B0+urH~$#K>:iSa-I-C#gDJ(9UUFubOeRHsDX3Ko`?T--iL+j".to_string(),
		    ":~vGLKg+n!*iJ\\.*wfxK)5Qmh%<:f^$nql7OB$}M/d.F,5[=>ZW*#f=0>Ao(@~lEHbSG1%,b_Uy2!zL%2GMB0O.t[#UcQ[]ufFZJ!K<kLgDNQlx)s8+75E^[-\"!1l[i<S#G\"B]xZ5?as*@8Laq`{@r".to_string(),
		    "myS=C|jEWnl,aC\\7!jv9[!vh/PAK}_H&<.o]qf|y@4L:?ssLg3N!v7/N5RyPHn=5%Fyh(4-Z:<6wf]^#t~0:i(X\\l-7]9olH9WLV~`L~JQ7ye7B1RSi2N$PuHwjj\\pb}J\\R~pe?j+X>R#p@MyqBBe*".to_string(),
		]
            );

            Ok(())
        }
    }
}
