
use super::ToAndFromFS;

impl ToAndFromFS for u8 {
    fn to_writer(&self, r: &mut impl std::io::Write) -> anyhow::Result<()> {
        r.write_all(self.to_le_bytes().as_slice())?;

        Ok(())
    }

    fn from_reader(r: &mut impl std::io::Read) -> anyhow::Result<Option<Self>> {
        let mut nu = [0u8; 1];
        r.read_exact(nu.as_mut_slice())?;

        Ok(Some(u8::from_le_bytes(nu)))
    }
}

impl ToAndFromFS for u16 {
    fn to_writer(&self, r: &mut impl std::io::Write) -> anyhow::Result<()> {
        r.write_all(self.to_le_bytes().as_slice())?;

        Ok(())
    }

    fn from_reader(r: &mut impl std::io::Read) -> anyhow::Result<Option<Self>> {
        let mut nu = [0u8; 2];
        r.read_exact(nu.as_mut_slice())?;

        Ok(Some(u16::from_le_bytes(nu)))
    }
}

impl ToAndFromFS for u32 {
    fn to_writer(&self, r: &mut impl std::io::Write) -> anyhow::Result<()> {
        r.write_all(self.to_le_bytes().as_slice())?;

        Ok(())
    }

    fn from_reader(r: &mut impl std::io::Read) -> anyhow::Result<Option<Self>> {
        let mut nu = [0u8; 4];
        r.read_exact(nu.as_mut_slice())?;

        Ok(Some(u32::from_le_bytes(nu)))
    }
}

impl ToAndFromFS for u64 {
    fn to_writer(&self, r: &mut impl std::io::Write) -> anyhow::Result<()> {
        r.write_all(self.to_le_bytes().as_slice())?;

        Ok(())
    }

    fn from_reader(r: &mut impl std::io::Read) -> anyhow::Result<Option<Self>> {
        let mut nu = [0u8; 8];
        r.read_exact(nu.as_mut_slice())?;

        Ok(Some(u64::from_le_bytes(nu)))
    }
}