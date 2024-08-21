pub mod roles {
    use crate::{
        api::{
            self,
            atlas::{
                requests::{GenerateCredentialsRequest, GenerateCredentialsRequestBuilder},
                responses::GenerateCredentialsResponse,
            },
        },
        client::Client,
        error::ClientError,
    };

    pub async fn credentials(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut GenerateCredentialsRequestBuilder>,
    ) -> Result<GenerateCredentialsResponse, ClientError> {
        let mut t = GenerateCredentialsRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();

        api::exec_with_result(client, endpoint).await
    }
}