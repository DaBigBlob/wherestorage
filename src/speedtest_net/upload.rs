
use crate::prelude::*;
use crate::speedtest_net::{request, net_utils::ChunkBytes, net_utils::ChunkJson};
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub async fn upload_file(file: PathBuf) -> Result<()> {
    let client = request::client()?;
    let _file_data = fs::read(file)
        .map_err(|e| Error::from_err(e))?;

    let _a = _file_data.into_iter();

    let a = upload_chunk(
        client,
        ChunkBytes::try_from(ChunkJson {
            server_id: 10200,
            ping: 65535,
            upload: 8388608,
            download: 8388608
        })?
    ).await?;
    println!("https://www.speedtest.net/result/{}", a);

    Ok(())
}

async fn upload_chunk(client: Client, payload: ChunkBytes) -> Result<u64>{
    client
    .post("https://www.speedtest.net/api/results.php")
    .body(ChunkJson::from(payload).to_string())
    .send().await
    .map_err(|e| Error::from_err(e))?
    .json::<Value>().await
    .map_err(|e| Error::from_err(e))?
    .get("resultid")
    .ok_or(Error::from_str("No 'resultid' in response"))?
    .as_u64()
    .ok_or(Error::from_str("'resultid' not u64"))
}


// struct UpChunkProcessed {
//     epoch_delta: u64,
//     can_be_cast_as: u8
// }

// const U8_MAX: u64 = u8::MAX as u64;
// const U16_MAX: u64 = u16::MAX as u64;
// const U32_MAX: u64 = u32::MAX as u64;

// fn up_process_chunk(epoch: u64, payload: u64) -> UpChunkProcessed {
//     let epoch_delta = up_load_chunk(payload) - epoch;
//     let can_be_cast_as: u8 = match epoch_delta {
//         ..=U8_MAX => 1<<0,
//         ..=U16_MAX => 1<<1,
//         ..=U32_MAX => 1<<2,
//         ..=u64::MAX => 1<<3
//     };

//     UpChunkProcessed { epoch_delta, can_be_cast_as }
// }

