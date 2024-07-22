
#[derive(Debug, Clone)]
struct ChunkBytes([u8; 9]);

#[derive(Debug, Clone)]
pub struct ChunkJson {
    server_id: u16,     // 500 to 65456            :::: 64957  states ::~15bits ::1.9 bytes::10bit ::1B + 7b
    ping: u16,          // 0 to 65536              :::: 65537  states ::~16bits ::2 bytes  ::16bit ::2B
    upload: u32,        // 1 to 9999999 (7 of them):::: 9999999states ::~23bits ::2.9 bytes::23bit ::2B + 7b
    download: u32,      // 1 to 9999999 (7 of them):::: 9999999states ::~23bits ::2.9 bytes::23bit ::2B + 7b
}                       // total log_2(9999999*9999999*65537*64957) = ~78 bits = ~9 bytes = 9*8 = 72 bits

impl From<ChunkBytes> for ChunkJson {
    /* byte allocation
        bytes[0]            -> server_id
        bytes[1,2]          -> ping
        bytes[3,4]          -> upload
        bytes[5,6]          -> download
        bytes[7] & 0b1      -> server_id
        bytes[7] >> 1       -> upload
        bytes[8] & 0b1      -> download
        bytes[8] >> 1       -> download
     */
    fn from(cb: ChunkBytes) -> Self {
        let server_id: u16
            = 500u16
            +  (((cb.0[0] as u16)        << 2) & 0b11111111_00u16)
            + ((((cb.0[7] as u16) & 0b1) << 1) & 0b1_0u16)
            +   ((cb.0[8] as u16) & 0b1);
        let ping: u16
            = (((cb.0[1] as u16) << 8) & 0b11111111_00000000u16)
            +  ((cb.0[2] as u16)       & 0b11111111u16);
        let upload: u32
            = 1u32
            + (((cb.0[3] as u32) << (8+7)) & 0b11111111_00000000_0000000u32)
            + (((cb.0[4] as u32) << 7    ) & 0b11111111_0000000u32)
            + (((cb.0[7] as u32) >> 1    ) & 0b1111111u32);
        let download: u32
            = 1u32
            + (((cb.0[5] as u32) << (8+7)) & 0b11111111_00000000_0000000u32)
            + (((cb.0[6] as u32) << 7    ) & 0b11111111_0000000u32)
            + (((cb.0[8] as u32) >> 1    ) & 0b1111111u32);
        
        Self {server_id, ping, upload, download}
    }
}


impl From<ChunkJson> for ChunkBytes {
    fn from(cj: ChunkJson) -> Self {
        let mut bytes: [u8; 9] = [0; 9];

        let mut server_id = cj.server_id - 500u16;
        bytes[8] += (server_id & 0b1).to_le_bytes()[0];
        server_id = (server_id >> 1) & 0b11111111_1u16;
        bytes[7] += (server_id & 0b1).to_le_bytes()[0];
        server_id = (server_id >> 1) & 0b11111111u16;
        bytes[0] += server_id.to_le_bytes()[0];

        let ping = cj.ping.to_le_bytes();
        bytes[2] += ping[0];
        bytes[1] += ping[1];

        let upload = cj.upload - 1;
        bytes[7] += ((upload << 1) & 0b11111110u32).to_le_bytes()[0];
        let upload_bytes = ((upload >> 7) & 0b11111111_11111111u32).to_le_bytes();
        bytes[4] += upload_bytes[0];
        bytes[3] += upload_bytes[1];


        let download = cj.download - 1;
        bytes[8] += ((download << 1) & 0b11111110u32).to_le_bytes()[0];
        let download_bytes = ((download >> 7) & 0b11111111_11111111u32).to_le_bytes();
        bytes[6] += download_bytes[0];
        bytes[5] += download_bytes[1];

        ChunkBytes(bytes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max() {
        let a = ChunkJson::from(ChunkBytes([u8::MAX; 9]));
        assert_eq!(a.upload, 2u32.pow(23) - 1 + 1);
        assert_eq!(a.upload, a.download);
    }

    #[test]
    fn min() {
        let b = ChunkJson::from(ChunkBytes([u8::MIN; 9]));
        assert_eq!(b.upload, 1);
        assert_eq!(b.upload, b.download);
    }

    #[test]
    fn good_enc_dec() {
        let cb = ChunkBytes([100, 101, 102, 103, 104, 105, 106, 107, 108]);
        let cj = ChunkJson::from(cb.clone());
        assert_eq!(ChunkBytes::from(cj).0, cb.0);
    }
}