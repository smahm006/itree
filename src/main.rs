use itree::{
    cli::*,
    config
};

use clap::Parser;

async fn entrypoint() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cfg = config::Config::load(cli.url, cli.token)?;
    let _ = itree::api::Client::new(&cfg)?;
     match cli.command {
         ItreeCommands::Config { command } => match command {
             ItreeConfigCommands::List  => config::list(cfg)?,
             ItreeConfigCommands::Get { header, key }  => config::get(cfg, header, key)?,
             ItreeConfigCommands::Set { header, key, value }  => config::set(cfg, header, key, value)?,
         },
         _ => todo!()
     }
    Ok(())
}

#[tokio::main]
async fn main() {
    match entrypoint().await {
        Ok(()) => (),
        Err(err) => {
            eprintln!("itree: {:#}\n\nRun 'itree --help' for more information", err);
            std::process::exit(1);
        }
    }
}
