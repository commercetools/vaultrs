use serde::{Deserialize, Serialize};

/// Response from executing
/// [GenerateCredentialsRequest][crate::api::atlas::requests::GenerateCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCredentialsResponse {
    pub public_key: String,
    pub private_key: String,
}
