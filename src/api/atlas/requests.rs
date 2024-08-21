use rustify_derive::Endpoint;
use std::fmt::Debug;

use super::responses::GenerateCredentialsResponse;

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    method = "GET",
    response = "GenerateCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}