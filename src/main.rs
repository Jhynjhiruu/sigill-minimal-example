use std::env::var;

use anyhow::Result;

use clap::{Parser, Subcommand};

use xshell::{cmd, Shell};

const PROG_DIR: &str = "min";
const PROG_PATH: &str = "target/mips-ultra64-cpu/release/minimal_example.rlib";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Profile to build with
    #[arg(default_value_t = String::from("release"))]
    profile: String,
}

#[derive(Subcommand)]
enum Command {
    /// Build payload and entry
    Build,

    /// Clean up build
    Clean,
}

fn build(profile: String) -> Result<()> {
    let sh = Shell::new()?;

    sh.remove_path(format!("./{PROG_DIR}/{PROG_PATH}"))?;

    let cargo = var("CARGO").unwrap_or("cargo".into());

    sh.change_dir(PROG_DIR);

    sh.set_var("RUST_BACKTRACE", "1");

    cmd!(
        sh,
        "{cargo} build -Z build-std=core --color always --profile {profile} -p minimal-example --release"
    )
    .run()?;

    sh.change_dir("..");

    Ok(())
}

fn clean(_profile: String) -> Result<()> {
    let sh = Shell::new()?;

    let cargo = var("CARGO").unwrap_or("cargo".into());

    sh.change_dir(PROG_DIR);

    cmd!(sh, "{cargo} clean").run()?;

    sh.change_dir("..");

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Build => build(cli.profile),
        Command::Clean => clean(cli.profile),
    }
}
