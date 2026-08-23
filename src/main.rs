use std::process::ExitCode;

use clap::Parser;
use rustagent::{cli, config, error::Error, request};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("rustagent: {err}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Error> {
    let args = cli::Args::parse();
    let cfg = config::Config::from_env()?;
    let client = request::Client::new(cfg)?;
    let _res = client.llm_request(args.task).await?;

    Ok(())
}
