#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut app = tide::new();

    app.at("/").get(|_| async move { Ok("Hello world!\n") });

    Ok(app.listen("127.0.0.1:8080").await?)
}
