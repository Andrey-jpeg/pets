use std::env;

use tide::Request;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(hello);
    
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

async fn hello(_req: Request<()>) -> tide::Result {
    Ok(format!("Hello World").into())
}