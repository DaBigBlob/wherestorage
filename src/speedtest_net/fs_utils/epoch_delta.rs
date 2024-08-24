
use super::ToAndFromFS;

impl ToAndFromFS for u8 {
    fn to_writer(&self, r: &mut impl std::io::Write) -> anyhow::Result<()> {
        todo!()
    }

    fn from_reader(r: &mut impl std::io::Read) -> anyhow::Result<Option<Self>> {
        todo!()
    }
}