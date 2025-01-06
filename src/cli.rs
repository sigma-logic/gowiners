use std::{env::current_dir, fs, io::ErrorKind};

use anyhow::anyhow;
use bpaf::*;
use colored::Colorize;

use crate::{GowinEda, Project, commands};

#[derive(Debug, Clone)]
pub enum Opts {
	Impl {},
}

fn options() -> OptionParser<Opts> {
	let impl_opts = construct!(Opts::Impl {})
		.to_options()
		.descr("Runs synthesis and routing");

	let impl_cmd = impl_opts.command("impl");

	construct!([impl_cmd])
		.to_options()
		.descr(env!("CARGO_PKG_DESCRIPTION"))
}

pub fn cli() -> anyhow::Result<()> {
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

	match opts {
		Opts::Impl { .. } => {
			println!("{}", "Run implementation".bold());
			let mut wrk = eda.pipeline();
			println!("{}", "  Evaluating project".bold());
			wrk.evaluate(&project)?;
			println!("{}", "  Run Gowin EDA syn and pnr tasks".bold());
			wrk.push(commands::Run::All);
			wrk.run()?;
			println!("{}", "Completed".bold().green());
		}
	}

	Ok(())
}
