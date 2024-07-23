
/*  next_delta from file_epoch
    0b0 << (8*1-1) :: 2^7 = 128 states
    0b1 << (8*4-1) :: 2^31 = 2147483648 states
*/

/* compression sequence
    has 2 phases:
        - UnitToIntrimFileDescriptor
        - IntrimFileDescriptorToFull
    8B
    -> 9B   :: 1u64 1u8 :: 2
    -> 9*2B :: 1u64 
*/

/*
    must start with 0b11111111u8
    size: 1+1+4+8 to 1+1+255+4+8 = 14 to 269 bytes
    so dont check for it if intrim file under 14bytes
*/
pub struct IntrimFileDeclaration {
    name: Option<String>,   // 0 to 255 bytes
    to_uncompressed: u32,   // static size
    file_epoch: u64         // static size
}

// pub enum IntrimFileWritable {
// }