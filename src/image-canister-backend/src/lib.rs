use candid::{ CandidType, Deserialize };
use candid::Principal;
use ic_cdk::api::call::{ CallResult, RejectionCode };
use serde::Serialize;
use serde_bytes::{ self, ByteBuf };

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct ImageData {
    pub content: Option<ByteBuf>,
    pub name: String,
    pub content_type: String,
    // you can add more params
}

type ReturnResult = Result<u32, String>;

#[ic_cdk::update]
pub async fn upload_image(canister_id: String, image_data: ImageData) -> Result<String, String> {
    let response: CallResult<(ReturnResult,)> = ic_cdk::call(
        Principal::from_text(canister_id).unwrap(),
        "create_file",
        (image_data,)
    ).await;

    let res0: Result<(Result<u32, String>,), (RejectionCode, String)> = response;

    let formatted_value = match res0 {
        Ok((Ok(value),)) => {
            format!("{}", value);
            Ok(format!("{}", value))
            // value
        }
        Ok((Err(err),)) => { Err(err) }
        Err((code, message)) => {
            match code {
                RejectionCode::NoError => Err("NoError".to_string()),
                RejectionCode::SysFatal => Err("SysFatal".to_string()),
                RejectionCode::SysTransient => Err("SysTransient".to_string()),
                RejectionCode::DestinationInvalid => Err("DestinationInvalid".to_string()),
                RejectionCode::CanisterReject => Err("CanisterReject".to_string()),
                _ => Err(format!("Unknown rejection code: {:?}: {}", code, message)),
            }
        }
    };

    formatted_value
}
