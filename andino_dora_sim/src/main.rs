// Inspired on https://github.com/dora-rs/dora/blob/bccb1ae27dbaf59d53818eae5241f8be70ad3082/examples/rust-dataflow/run.rs
//
use dora_tracing::set_up_tracing;
use eyre::{Context, bail};
use std::path::Path;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    set_up_tracing("andino_dora_sim").wrap_err("failed to set up tracing subscriber")?;

    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let graphs_dir = cargo_dir.join("graphs");
    std::env::set_current_dir(graphs_dir).wrap_err("failed to set working dir")?;

    let args: Vec<String> = std::env::args().collect();
    let dataflow = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new("mujoco_teleoperation.yml")
    };

    build_dataflow(dataflow).await?;

    run_dataflow(dataflow).await?;

    Ok(())
}

async fn build_dataflow(dataflow: &Path) -> eyre::Result<()> {
    let mut cmd = tokio::process::Command::new("dora");
    cmd.arg("build").arg(dataflow);
    if !cmd.status().await?.success() {
        bail!("failed to build dataflow");
    };
    Ok(())
}

async fn run_dataflow(dataflow: &Path) -> eyre::Result<()> {
    let mut cmd = tokio::process::Command::new("dora");
    cmd.arg("run").arg(dataflow);
    if !cmd.status().await?.success() {
        bail!("failed to run dataflow");
    };
    Ok(())
}
