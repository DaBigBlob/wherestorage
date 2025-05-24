
use anyhow::bail;
use super::ToAndFromFS;

/** 
 * u8: 0b00
 * u16: 0b01
 * u32: 0b10
 * u64: 0b11
 */

pub trait UintSignature {

}

impl ToAndFromFS for u64 {
    fn to_writer(&self, r: &mut impl std::io::Write) -> anyhow::Result<()> {
        let mut bs = self.to_be_bytes();
        bs[0] = bs[0] & 0b1111u8;
        r.write_all(bs.as_slice())?;

        Ok(())
    }

    fn from_reader(r: &mut impl std::io::Read) -> anyhow::Result<Option<Self>> {
        let mut nu = [0u8; 8];
        r.read_exact(nu.as_mut_slice())?;

        Ok(Some(u64::from_be_bytes(nu)))
    }
}