
use anyhow::{bail, Context, Result};
use std::io;
use super::ToAndFromFS;

/** Serialization Strategy
 * isFDFlag: 0xffu8
 * name_len: u8
 * name: \[u8]
 * size: \[u8; 8]
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
}

impl ToAndFromFS for FileDeclaration {
    fn to_writer(&self, r: &mut impl io::Write) -> Result<()> {
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

    /// we shall test for Result<Some> till we get the FD
    fn from_reader(r: &mut impl io::Read) -> Result<Option<Self>> {
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
