//! Struct that extract part of file (called block) and read it as vcf file.

/* std use */

/* crate use */
use bstr::ByteSlice as _;

/* project use */
use crate::block;
use crate::error;
use crate::impl_producer;
use crate::impl_reader;

/* mod declaration */
pub mod header;

/// Struct that store a VCF record
pub struct Record<'a> {
    /// Chromosome name
    pub chromosome: &'a [u8],

    /// Position
    pub position: &'a [u8],

    /// Identifiant
    pub identifiant: &'a [u8],

    /// Reference sequence
    pub reference: &'a [u8],

    /// Alternative sequence
    pub alternative: &'a [u8],

    /// Quality of variant
    pub quality: &'a [u8],

    /// Filter
    pub filter: &'a [u8],

    /// Info
    pub info: &'a [u8],

    /// Format
    pub format: &'a [u8],

    /// Genotype
    pub genotype: &'a [u8],
}

impl<'a> Record<'a> {
    /// Build a record from a line
    pub fn from_line(line: &'a [u8]) -> error::Result<Self> {
        let mut spliter = line.splitn_str(9, "\t");

        Ok(Record {
            chromosome: spliter.next().ok_or(error::Error::PartialRecord)?,
            position: spliter.next().ok_or(error::Error::PartialRecord)?,
            identifiant: spliter.next().ok_or(error::Error::PartialRecord)?,
            reference: spliter.next().ok_or(error::Error::PartialRecord)?,
            alternative: spliter.next().ok_or(error::Error::PartialRecord)?,
            quality: spliter.next().ok_or(error::Error::PartialRecord)?,
            filter: spliter.next().ok_or(error::Error::PartialRecord)?,
            info: spliter.next().ok_or(error::Error::PartialRecord)?,
            format: spliter.next().ok_or(error::Error::PartialRecord)?,
            genotype: spliter.next().ok_or(error::Error::PartialRecord)?,
        })
    }
}

impl_producer!(Producer, |block: &[u8]| {
    let mut end = block.len();

    end = block[..end]
        .rfind_byte(b'\n')
        .ok_or(error::Error::NoNewLineInBlock)?;

    if end < block.len() && block[end] == b'\n' {
        return Ok((end + 1) as u64);
    }

    Err(error::Error::NotAVcfFile)
});

impl_reader!(
    Reader,
    'a,
    Record,
    |block: &'a block::Block, offset: &mut usize| {
    if *offset == block.len() {
        Ok(None)
    } else {
        Ok(Some(Record::from_line(&block.data()[Self::get_line(block, offset)?])?))
    }
    }
);
