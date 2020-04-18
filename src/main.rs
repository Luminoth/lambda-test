use anyhow::bail;
use lambda::lambda;
//use log::error;
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

fn handler(e: CustomEvent) -> anyhow::Result<CustomOutput> {
    simple_logger::init_with_level(log::Level::Info)?;

    if e.first_name == "" {
        //error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

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
