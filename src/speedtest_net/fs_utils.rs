
/*  http3 style numbers as epoch_delta
    u8, u16, u32, u64 (is epoch)
    0b00 << (8*1-2) :: 2^6  = 64 states
    0b01 << (8*2-2) :: 2^14 = 16384 states
    0b10 << (8*4-2) :: 2^30 = 1073741824
        [10,133,099,161,583,616 bytes = 10 Exabytes]
    0b11 << (8*8-2) :: 2^62 = 4611686018427387904 states [is epoch]
*/

// pub struct IntrimFileDescriptor {
//     to_uncompressed: u64,
//     name: Option<String>
// }

// pub enum IntrimFileWritable {

// }