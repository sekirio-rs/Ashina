use argh::FromArgs;
use simple_logger::SimpleLogger;
use std::{env, path::Path, process::Command as PCommand};

/// Ashina build tools
#[derive(Debug, FromArgs)]
struct Options {
    #[argh(subcommand)]
    command: Command,

    /// verbose log
    #[argh(switch)]
    verbose: bool,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Build(BuildCmd),
    Test(TestCmd),
    Bench(BenchCmd),
}

/// Build Command
#[derive(FromArgs, Debug, Default)]
#[argh(subcommand, name = "build")]
struct BuildCmd {
    /// build target package
    #[argh(option)]
    pub package: Option<String>,

    /// specify release mode
    #[argh(switch)]
    release: bool,
}

/// Test Command
#[derive(FromArgs, Debug, Default)]
#[argh(subcommand, name = "test")]
struct TestCmd {}

/// Bench Command
#[derive(FromArgs, Debug, Default)]
#[argh(subcommand, name = "bench")]
struct BenchCmd {}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    SimpleLogger::new()
        .with_level(if options.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init()?;

    log::debug!("{:?}", options);

    match options.command {
        Command::Build(cmd) => {
            if let Some(package) = cmd.package {
                match package.as_str() {
                    "ashina" => ashina::build(cmd.release)?,
                    name => {
                        log::debug!("unknown package: {}", name);
                    }
                }
            }
        }
        Command::Test(_cmd) => {}
        Command::Bench(_cmd) => {}
    }

    Ok(())
}

struct XTask;

impl XTask {
    pub fn build(package: &str, is_release: bool) -> anyhow::Result<()> {
        let root = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf();
        let cargo = env::var("CARGO").unwrap_or("cargo".to_string());

        let mut cargo = PCommand::new(&cargo);
        cargo.current_dir(root.join(package));
        cargo.arg("build");
        if is_release {
            cargo.arg("--release");
        }

        let _status = cargo.status()?;
        Ok(())
    }
}

mod ashina {
    pub fn build(is_release: bool) -> anyhow::Result<()> {
        crate::XTask::build("ashina", is_release)
    }
}
