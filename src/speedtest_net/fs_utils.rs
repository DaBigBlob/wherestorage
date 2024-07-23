
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

// pub enum IntrimFileWritable {
// }