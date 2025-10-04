mod config;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::load("config.yml")?;

    dbg!(&cfg);

    Ok(())
}