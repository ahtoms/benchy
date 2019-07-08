
use serde::{Serialize, Deserialize};
/// Route Objects (ROBJS)

///SubmissionRequest object which contains
///Username and data which is a base64 string
///TODO: Move to separate file
#[derive(Deserialize, Serialize)]
pub struct SubmissionRequest{
    pub ident: String,
    pub data: String,
}

///Just a simple response to pass back to the
#[derive(Serialize)]
pub struct SubmissionResponse {
    pub result: u8,
}
