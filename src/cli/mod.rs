use std::{env::current_dir, fs, io::ErrorKind};

use anyhow::anyhow;
use bpaf::*;
use colored::Colorize;

use crate::wrapper::{eda::GowinEda, pipeline::commands, project::Project};

#[derive(Debug, Clone)]
pub enum Opts {
	Impl {},
	Flash { preset: String },
}

fn options() -> OptionParser<Opts> {
	let impl_opts = construct!(Opts::Impl {})
		.to_options()
		.descr("Runs synthesis and routing");

	let impl_cmd = impl_opts.command("impl");

	let preset = positional::<String>("PRESET");

	let flash_opts = construct!(Opts::Flash { preset })
		.to_options()
		.descr("Flashes bitstream using provided preset");

	let flash_cmd = flash_opts.command("flash");

	construct!([impl_cmd, flash_cmd])
		.to_options()
		.descr(env!("CARGO_PKG_DESCRIPTION"))
}

pub fn run() -> anyhow::Result<()> {
	let opts = options().fallback_to_usage().run();
	let cwd = current_dir()?;

	let gowiners_toml_str = match fs::read_to_string(cwd.join("Gowiners.toml")) {
		Ok(it) => it,
		Err(err) if err.kind() == ErrorKind::NotFound => {
			return Err(anyhow!("The `Gowiners.toml` file not found"));
		}
		Err(err) => Err(err)?,
	};

	let eda = GowinEda::from_env()
		.or_else(|| GowinEda::from_file(cwd.join(".gowin")))
		.ok_or(anyhow!("Please specify path to Gowin EDA installation dir via `GOWIN_EDA_HOME` environment variable of `.gowin` file"))?;

	let project: Project = toml::from_str(&gowiners_toml_str)?;

	let run_opts = || -> anyhow::Result<()> {
		match opts {
			Opts::Impl { .. } => {
				println!("{}", "Run implementation".bold());
				let mut wrk = eda.pipeline();
				println!("  {}", "Configuring project".bold());
				wrk.configure(&project)?;
				println!("  {}", "Run Gowin EDA syn and pnr tasks".bold());
				wrk.push(commands::Run::All);
				wrk.run()?;
			}
			Opts::Flash { preset } => {
				println!("{}", "Flash bitstream".bold());
				let mut prg = eda.programmer();
				println!("  {}", "Configuring programmer".bold());
				println!("    {} {}", "Preset:".bold(), preset.cyan());
				prg.evaluate(&project, preset)?;
				println!("  {}", "Flashing".bold());
				prg.run()?;
			}
		}

		Ok(())
	};

	match run_opts() {
		Ok(_) => {
			println!("{}", "Completed".bold().green());
		}
		Err(err) => {
			println!("{}", format!("Failed: {}", err).bold().red());
		}
	}

	Ok(())
}
