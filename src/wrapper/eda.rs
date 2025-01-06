use std::{
	env, fs,
	path::{Path, PathBuf},
};

use cfg_if::cfg_if;

use super::pipeline::Pipeline;
use crate::wrapper::programmer::Programmer;

#[derive(Debug, Clone)]
pub struct GowinEda {
	pub home: PathBuf,
}

impl GowinEda {
	pub fn new(home: impl Into<PathBuf>) -> Self {
		Self { home: home.into() }
	}

	pub fn from_env() -> Option<Self> {
		let os_path = env::var_os("GOWIN_EDA_HOME")?;
		Some(Self::new(os_path))
	}

	pub fn from_file(path: impl AsRef<Path>) -> Option<Self> {
		let path = fs::read_to_string(path).ok()?;
		Some(Self::new(path))
	}

	pub fn pipeline(&self) -> Pipeline {
		let bin_path = self.home.join(format!("IDE/bin/gw_sh{BIN_SUFFIX}"));
		Pipeline::new(&bin_path)
	}

	pub fn programmer(&self) -> Programmer {
		let bin_path = self
			.home
			.join(format!("Programmer/bin/programmer_cli{BIN_SUFFIX}"));
		Programmer::new(&bin_path)
	}
}

cfg_if! {
	if #[cfg(target_os = "windows")] {
		const BIN_SUFFIX: &str = ".exe";
	} else {
		const BIN_SUFFIX: &str = "";
	}
}
