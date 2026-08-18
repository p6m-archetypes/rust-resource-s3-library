pub mod settings;

use anyhow::Result;
use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::Client;
use settings::StorageS3Settings;

pub use aws_sdk_s3::Client as S3Client;

pub async fn connect(settings: &StorageS3Settings) -> Result<Client> {
    let credentials = Credentials::from_keys(&settings.access_key, &settings.secret_key, None);
    let config = Builder::new()
        .endpoint_url(&settings.endpoint)
        .region(Region::new("us-east-1"))
        .credentials_provider(credentials)
        .force_path_style(true)
        .build();
    let client = Client::from_conf(config);
    tracing::info!("S3 client connected to {}", settings.endpoint);
    Ok(client)
}
