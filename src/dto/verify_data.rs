use log;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyDataDto{
    pub error: Option<String>,
    pub data: Option<Data>, 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data{
    pub id: String,
    pub data: String,
    #[serde(rename(deserialize = "merkleRoot"))]
    pub merkle_root: String,
    #[serde(rename(deserialize = "txHash"))]
    pub tx_hash: String,
    #[serde(rename(deserialize = "blockNumber"))]
    pub block_number: String,
    pub timestamp: String,
    #[serde(rename(deserialize = "TSAGenTime"))]
    pub tsa_gen_time: String
}

// pub fn fetch_verify_data(token_id: String) -> UseStateHandle<Option<Vec<Video>>> {
//     let videos = use_state(|| None);

//     {
//         let videos = videos.clone();
//         use_effect_with_deps(
//             move |_| {
//                 let videos = videos.clone();
//                 spawn_local(async move {
//                     let fetch_videos: Vec<Video> =
//                         Request::get(&format!("https://yew.rs/tutorial/{}", token_id))
//                             .send()
//                             .await
//                             .unwrap()
//                             .json()
//                             .await
//                             .unwrap();
//                     videos.set(Some(fetch_videos));
//                 });
//                 || ()
//             },
//             (),
//         );
//     }
//     videos
    
// }
