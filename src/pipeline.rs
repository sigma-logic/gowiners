use std::{
	io,
	io::Write,
	path::{Path, PathBuf},
	process::{Command, ExitCode, Stdio},
};

use thiserror::Error;
use tracing::error;

use crate::{MaybeList, Project};

#[derive(Debug)]
pub struct TclCommand {
	name: String,
	flags: Vec<(String, String)>,
	args: Vec<String>,
}

impl TclCommand {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			flags: Vec::with_capacity(1),
			args: Vec::with_capacity(1),
		}
	}

	pub fn push_flag(
		&mut self,
		name: impl Into<String>,
		value: impl Into<String>,
	) {
		self.flags.push((name.into(), value.into()));
	}

	pub fn flag(
		mut self,
		name: impl Into<String>,
		value: impl Into<String>,
	) -> Self {
		self.push_flag(name, value);
		self
	}

	pub fn push_arg(&mut self, arg: impl Into<String>) {
		self.args.push(arg.into());
	}

	pub fn arg(mut self, arg: impl Into<String>) -> Self {
		self.push_arg(arg);
		self
	}

	pub fn build(self) -> String {
		let mut command = self.name.clone();
		command.push(' ');

		for (name, value) in self.flags {
			command.push_str(&format!("-{name} {value} "));
		}

		for arg in self.args {
			command.push_str(&arg);
			command.push(' ');
		}

		command.pop();

		command
	}
}

pub mod commands {
	use super::TclCommand;

	#[derive(Debug)]
	pub struct AddFiles<'a> {
		pub filetype: &'a str,
		pub files: Vec<String>,
	}

	impl From<AddFiles<'_>> for TclCommand {
		fn from(value: AddFiles<'_>) -> Self {
			let mut tcl = TclCommand::new("add_file");

			tcl.push_flag("type", value.filetype);

			for file in value.files {
				tcl.push_arg(file);
			}

			tcl
		}
	}

	#[derive(Debug)]
	pub struct SetDevice<'a> {
		pub family: &'a str,
		pub part: &'a str,
	}

	impl From<SetDevice<'_>> for TclCommand {
		fn from(value: SetDevice<'_>) -> Self {
			TclCommand::new("set_device")
				.flag("name", value.family)
				.arg(value.part)
		}
	}

	#[derive(Debug)]
	pub struct SetOption<'a> {
		pub name: &'a str,
		pub value: &'a str,
	}

	impl From<SetOption<'_>> for TclCommand {
		fn from(value: SetOption<'_>) -> Self {
			TclCommand::new("set_option").flag(value.name, value.value)
		}
	}

	#[derive(Debug, Copy, Clone)]
	pub enum Run {
		Syn,
		Pnr,
		All,
	}

	impl From<Run> for TclCommand {
		fn from(value: Run) -> Self {
			TclCommand::new("run").arg(match value {
				Run::Syn => "syn",
				Run::Pnr => "pnr",
				Run::All => "all",
			})
		}
	}
}

#[derive(Debug)]
pub struct Pipeline {
	bin: PathBuf,
	list: Vec<TclCommand>,
}

impl Pipeline {
	pub fn new(bin: impl Into<PathBuf>) -> Self {
		Self {
			bin: bin.into(),
			list: Vec::new(),
		}
	}

	pub fn push(&mut self, tcl: impl Into<TclCommand>) {
		self.list.push(tcl.into());
	}

	pub fn run(self) -> Result<(), PipelineError> {
		let mut shell = Command::new(&self.bin)
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.stdin(Stdio::piped())
			.spawn()
			.map_err(PipelineError::Spawn)?;

		let mut stdin = shell.stdin.take().expect("No stdin");

		let line = self
			.list
			.into_iter()
			.map(|it| it.build() + ";")
			.collect::<String>();

		stdin.write_all(line.as_bytes())?;
		drop(stdin);

		shell.wait()?;

		Ok(())
	}
}

impl Pipeline {
	pub fn evaluate(&mut self, project: &Project) -> Result<(), EvaluationError> {
		self.push(commands::SetDevice {
			family: &project.device.family,
			part: &project.device.part,
		});

		self.push(commands::SetOption {
			name: if project.hdl.standard.starts_with("vhdl") {
				"vhdl_std"
			} else {
				"verilog_std"
			},
			value: &project.hdl.standard,
		});

		self.push(commands::SetOption {
			name: "top_module",
			value: project.hdl.top.as_ref().unwrap_or(&"top".to_owned()),
		});

		fn stringify_path<T: AsRef<Path>>(
			path: T,
		) -> Result<String, EvaluationError> {
			let path = path.as_ref().canonicalize()?;
			Ok(path.to_string_lossy().into_owned())
		}

		for (filetype, list) in project.files.iter() {
			let paths = match list {
				MaybeList::Single(it) => {
					vec![stringify_path(it)?]
				}
				MaybeList::List(it) => it
					.iter()
					.map(stringify_path)
					.collect::<Result<Vec<_>, _>>()?,
			};

			self.push(commands::AddFiles {
				filetype,
				files: paths,
			});
		}

		Ok(())
	}
}

#[derive(Error, Debug)]
pub enum EvaluationError {
	#[error(transparent)]
	Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum PipelineError {
	#[error("Failed to spawn pipeline shell")]
	Spawn(#[source] io::Error),

	#[error("Pipeline run failed with exit code: {0:?}")]
	Run(ExitCode),

	#[error(transparent)]
	Io(#[from] io::Error),
}
