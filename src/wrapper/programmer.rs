use std::{
	collections::HashMap,
	io,
	path::PathBuf,
	process::{Command, Stdio},
};

use thiserror::Error;

use crate::wrapper::project::Project;

#[derive(Debug)]
pub struct Programmer {
	bin: PathBuf,
	args: HashMap<String, String>,
}

impl Programmer {
	pub fn new(bin: impl Into<PathBuf>) -> Self {
		Self {
			bin: bin.into(),
			args: HashMap::with_capacity(4),
		}
	}

	pub fn evaluate(
		&mut self,
		project: &Project,
		preset: impl AsRef<str>,
	) -> Result<(), EvaluationError> {
		let cfg = &project.programmer;

		let mut insert =
			|args: &HashMap<String, String>| -> Result<(), EvaluationError> {
				for (arg, value) in args {
					match arg.as_ref() {
						"op" => {
							self.args.insert("run".into(), value.clone());
						}
						"fs" => {
							self
								.args
								.insert(arg.clone(), parse_and_canonicalize(value)?);
						}
						_ => {}
					};
				}

				Ok(())
			};

		insert(&cfg.args)?;

		let preset = preset.as_ref();
		let preset = cfg
			.preset
			.get::<str>(preset.as_ref())
			.ok_or_else(|| EvaluationError::NoSuchPreset(preset.to_owned()))?;

		insert(&preset.args)?;

		self
			.args
			.insert("device".into(), project.device.family.clone());

		Ok(())
	}

	pub fn run(&mut self) -> Result<(), ProgrammerError> {
		let exit_code = Command::new(&self.bin)
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.args(self.args.iter().map(|(k, v)| format!("--{k}={v}")))
			.spawn()
			.map_err(ProgrammerError::Spawn)?
			.wait()?;

		if !exit_code.success() {
			return Err(ProgrammerError::Failed);
		}

		Ok(())
	}
}

fn parse_and_canonicalize(str: &str) -> Result<String, EvaluationError> {
	Ok(
		PathBuf::from(str)
			.canonicalize()?
			.to_string_lossy()
			.into_owned(),
	)
}

#[derive(Error, Debug)]
pub enum EvaluationError {
	#[error(transparent)]
	Io(#[from] io::Error),

	#[error("No such preset: {0}")]
	NoSuchPreset(String),
}

#[derive(Error, Debug)]
pub enum ProgrammerError {
	#[error("Failed to spawn pipeline shell")]
	Spawn(#[source] io::Error),

	#[error("Programmer exited with no success")]
	Failed,

	#[error(transparent)]
	Io(#[from] io::Error),
}
