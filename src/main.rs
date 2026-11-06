use ::authentication::{authentication, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = authentication::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.start().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.stop().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-authentication operational");
    Ok(())
}
