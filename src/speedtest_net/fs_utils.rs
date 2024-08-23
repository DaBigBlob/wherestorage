use anyhow::{bail, Context, Result};
use std::io;

/*  next_delta from file_epoch
    0b0 << (8*1-1) :: 2^7 = 128 states
    0b1 << (8*4-1) :: 2^31 = 2147483648 states
*/

/* compression sequence
    has 2 phases:
        - UnitToIntrimFileDescriptor
        - IntrimFileDescriptorToFull
    [decompression]
    8B (unit)
    -> 9B   :: 1u64 1u8 :: 2
    -> 9*2B :: 1u64
*/

/* plan [FINAL TILL NOW]
    have a file_descriptor that states:
        - name of file
        - size of file
        - itrs to fully decompressed
        - etc.
    this almost definitely has a size more than 9 bytes (chunk)
    this descriptor startes with 0b11111111u8
    [while decompressing]
    we start at unit
    we keep decompressing while ignoring the 1st byte as compressed data
    but checking the 1st byte for 0b11111111u8
    after receiving the file_descriptor, we comtinue as normal till fully decompressed
    [while compressing]
    we already know the contents of the FD so no need to bother
*/

/*
    must start with 0b11111111u8
    size: 1+1+4+8 to 1+1+255+4+8 = 14 to 269 bytes
    so dont check for it if intrim file under 14bytes
*/
#[allow(dead_code)]
pub struct FileDeclaration {
    pub name: Option<String>, // 0 to 255 bytes
    /// number of file bytes
    pub size: u64,
}

#[allow(dead_code)]
impl FileDeclaration {
    pub fn new(name: Option<String>, size: u64) -> Result<Self> {
        if name.as_ref().is_some_and(|n| n.len() > u8::MAX.into()) {
            bail!("File name too big (max 255 bytes)")
        } else {
            Ok(FileDeclaration { name, size })
        }
    }

    pub fn to_writer(&self, r: &mut impl io::Write) -> Result<()> {
        let mut v = Vec::new();
        v.push(u8::MAX); // declaration
        match &self.name {
            // deal with the name
            None => v.push(0),
            Some(s) => {
                v.push(s.len().to_le_bytes()[0]);
                v.extend_from_slice(s.as_bytes());
            }
        };
        v.extend_from_slice(self.size.to_le_bytes().as_slice());
        r.write_all(v.as_slice()).context("unable to write_all")
    }

    pub fn to_writer_flushed(self, r: &mut impl io::Write) -> Result<()> {
        self.to_writer(r)?;
        r.flush().context("unable to flush writer")
    }

    pub fn from_reader(r: &mut impl io::Read) -> Result<Option<Self>> {
        if {
            let mut sign = [0u8; 1];
            r.read_exact(sign.as_mut_slice())?;
            sign != [0b11111111u8]
        } {Ok(None)}
        else {
            let mut name_len = [0u8; 1];
            r.read_exact(name_len.as_mut_slice())?;

            let name: Option<String> = match name_len[0] {
                0 => None,
                s => {
                    let mut name_bytes = vec![0; s as usize];
                    r.read_exact(name_bytes.as_mut_slice())?;
                    Some(String::from_utf8(name_bytes)?)
                }
            };

            let size: u64 = {
                let mut size_bytes = [0u8; 8];
                r.read_exact(size_bytes.as_mut_slice())?;
                u64::from_le_bytes(size_bytes)
            };

            Ok(Some(Self { name, size }))
        }
    }
}
