use anyhow::bail;
use lambda::lambda;
//use log::error;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_s3::S3Client;
use serde::{Deserialize, Serialize};
use simple_logger;

#[derive(Deserialize)]
struct CustomEvent {
    first_name: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

#[lambda]
#[tokio::main]
async fn main(e: CustomEvent) -> anyhow::Result<CustomOutput> {
    handler(e)
}

fn validate_name<S>(_dynamodb: &DynamoDbClient, first_name: S) -> anyhow::Result<()>
where
    S: AsRef<str>,
{
    if first_name.as_ref() == "" {
        //error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    // TODO: verify it with the database

    Ok(())
}

fn upload_data(_s3: &S3Client) -> anyhow::Result<()> {
    // TODO: upload the datas

    Ok(())
}

fn handler(e: CustomEvent) -> anyhow::Result<CustomOutput> {
    simple_logger::init_with_level(log::Level::Info)?;

    let dynamodb = DynamoDbClient::new(Region::UsWest2);
    validate_name(&dynamodb, &e.first_name)?;

    let s3 = S3Client::new(Region::UsWest2);
    upload_data(&s3)?;

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler() {
        let output = handler(CustomEvent {
            first_name: "Shane".to_owned(),
        })
        .unwrap();
        assert_eq!(output.message, "Hello, Shane!");
    }
}
