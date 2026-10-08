use clap::{Parser, ValueEnum};
use tapo::ApiClient;

#[derive(Clone, ValueEnum)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Parser)]
struct Args {
    /// Action à effectuer
    action: Action,

    /// Adresse IP de la prise
    #[arg(long, env = "TAPO_IP")]
    ip: String,

    #[arg(long, env = "TAPO_USERNAME")]
    username: String,

    #[arg(long, env = "TAPO_PASSWORD", hide_env_values = true)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    let device = ApiClient::new(args.username, args.password)
        .p100(&args.ip)
        .await?;

    match args.action {
        Action::On => device.on().await?,
        Action::Off => device.off().await?,
        Action::Toggle => {
            let info = device.get_device_info().await?;
            if info.device_on {
                device.off().await?
            } else {
                device.on().await?
            }
        }
    }

    Ok(())
}
