
// use crate::prelude::*;
// use serde::Serialize;

// #[derive(Serialize)]
// pub struct UpFinalPayload {
//     serverid: u16,      // 500 to 65456             :::: 64957    states :: ~15 bits   :: 1.9 bytes
//     ping: u16,          // 0 to 65536               :::: 65537    states :: ~16 bits   :: 2 bytes
//     upload: u32,        // 1 to 9999999 (7 of them) :::: 9999999  states :: ~23 bits   :: 2.9 bytes
//     download: u32,      // 1 to 9999999 (7 of them) :::: 9999999  states :: ~23 bits   :: 2.9 bytes
//     hash: String                // total log_2(9999999*9999999*65537*64957) = ~78 bits = ~9 bytes 
// }

// impl UpFinalPayload {
//     pub fn new(
//         serverid: u16,
//         ping: u16,  
//         upload: u32,
//         download: u32
//     ) -> Self {
//         UpFinalPayload {
//             serverid,
//             ping,
//             upload,
//             download,
//             hash: format!("{:x}", md5::compute(format!("{}-{}-{}-817d699764d33f89c", ping, upload, download)))
//         }
//     }
//     pub fn new_json_string(self) -> Result<String> {
//         serde_json::to_string(&self).map_err(|e| Error::UploadJsonBuildError(e))
//     }
// }
