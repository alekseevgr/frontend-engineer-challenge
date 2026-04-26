use std::error::Error;
use api::server::run;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await?;
    Ok(())
}
