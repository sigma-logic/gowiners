use std::{
	io,
	io::Write,
	path::{Path, PathBuf},
	process::{Command, ExitCode, Stdio},
};

use thiserror::Error;
use tracing::error;

use super::project::{MaybeList, Project};
use crate::pipeline::commands::ProjectOption;

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
	pub struct AddFiles {
		pub filetype: String,
		pub files: Vec<String>,
	}

	impl From<AddFiles> for TclCommand {
		fn from(value: AddFiles) -> Self {
			let mut tcl = TclCommand::new("add_file");

			tcl.push_flag("type", value.filetype);

			for file in value.files {
				tcl.push_arg(file);
			}

			tcl
		}
	}

	#[derive(Debug)]
	pub struct SetDevice {
		pub family: String,
		pub part: String,
	}

	impl From<SetDevice> for TclCommand {
		fn from(value: SetDevice) -> Self {
			TclCommand::new("set_device")
				.flag("name", value.family)
				.arg(value.part)
		}
	}

	#[derive(Debug, Clone)]
	pub struct ProjectOption {
		pub name: String,
		pub value: String,
	}

	#[derive(Debug, Clone, Default)]
	pub struct SetOptions(pub Vec<ProjectOption>);

	impl SetOptions {
		pub fn push(&mut self, option: impl Into<ProjectOption>) {
			self.0.push(option.into());
		}
	}

	impl From<SetOptions> for TclCommand {
		fn from(value: SetOptions) -> Self {
			let mut command = TclCommand::new("set_option");

			for option in value.0 {
				command.push_flag(option.name, option.value);
			}

			command
		}
	}

	impl From<ProjectOption> for TclCommand {
		fn from(value: ProjectOption) -> Self {
			SetOptions(vec![value]).into()
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

mod options {
	use super::commands::ProjectOption;

	#[derive(Debug, Clone)]
	pub struct VerilogStd(pub String);

	impl From<VerilogStd> for ProjectOption {
		fn from(value: VerilogStd) -> Self {
			ProjectOption {
				name: "verilog_std".into(),
				value: value.0,
			}
		}
	}

	#[derive(Debug, Clone)]
	pub struct VhdlStd(pub String);

	impl From<VhdlStd> for ProjectOption {
		fn from(value: VhdlStd) -> Self {
			ProjectOption {
				name: "vhdl_std".into(),
				value: value.0,
			}
		}
	}

	#[derive(Debug, Clone)]
	pub struct IncludePath(pub String);

	impl From<IncludePath> for ProjectOption {
		fn from(value: IncludePath) -> Self {
			ProjectOption {
				name: "include_path".into(),
				value: value.0,
			}
		}
	}

	#[derive(Debug, Clone)]
	pub struct TopModule(pub String);

	impl From<TopModule> for ProjectOption {
		fn from(value: TopModule) -> Self {
			ProjectOption {
				name: "top_module".into(),
				value: value.0,
			}
		}
	}

	#[derive(Debug, Copy, Clone)]
	pub struct PlaceOption(pub u32);

	impl From<PlaceOption> for ProjectOption {
		fn from(value: PlaceOption) -> Self {
			ProjectOption {
				name: "place_option".into(),
				value: value.0.to_string(),
			}
		}
	}

	#[derive(Debug, Copy, Clone)]
	pub struct RouteOption(pub u32);

	impl From<RouteOption> for ProjectOption {
		fn from(value: RouteOption) -> Self {
			ProjectOption {
				name: "route_option".into(),
				value: value.0.to_string(),
			}
		}
	}

	#[derive(Debug, Copy, Clone)]
	pub struct ReplicateResources(pub bool);

	impl From<ReplicateResources> for ProjectOption {
		fn from(value: ReplicateResources) -> Self {
			ProjectOption {
				name: "replicate_resources".into(),
				value: String::from(if value.0 { "1" } else { "0" }),
			}
		}
	}

	#[derive(Debug, Copy, Clone)]
	pub struct BitstreamCompress(pub bool);

	impl From<BitstreamCompress> for ProjectOption {
		fn from(value: BitstreamCompress) -> Self {
			ProjectOption {
				name: "bit_compress".into(),
				value: String::from(if value.0 { "1" } else { "0" }),
			}
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
	pub fn configure(
		&mut self,
		project: &Project,
	) -> Result<(), EvaluationError> {
		self.push(commands::SetDevice {
			family: project.device.family.clone(),
			part: project.device.part.clone(),
		});

		let mut options: commands::SetOptions = Default::default();

		options.push(if project.hdl.standard.starts_with("vhdl") {
			ProjectOption::from(options::VhdlStd(project.hdl.standard.clone()))
		} else {
			ProjectOption::from(options::VerilogStd(project.hdl.standard.clone()))
		});

		if let Some(include_dirs) = &project.hdl.include {
			let pathlist = include_dirs
				.iter()
				.map(stringify_path)
				.collect::<Result<Vec<String>, _>>()?
				.join(";");

			options.push(options::IncludePath(pathlist));
		}

		options.push(options::TopModule(
			project.hdl.top.clone().unwrap_or("top".to_owned()),
		));

		if let Some(pnr) = &project.pnr {
			options.push(options::PlaceOption(pnr.place_mode.unwrap_or(0)));
			options.push(options::RouteOption(pnr.route_mode.unwrap_or(0)));
			options.push(options::ReplicateResources(
				pnr.replicate.unwrap_or_default(),
			));
		}

		if let Some(bitstream) = &project.bitstream {
			options.push(options::BitstreamCompress(
				bitstream.compress.unwrap_or_default(),
			));
		}

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
				filetype: filetype.clone(),
				files: paths,
			});
		}

		self.push(options);

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
	#[error("Failed to spawn pipeline shell: {0}")]
	Spawn(#[source] io::Error),

	#[error("Pipeline run failed with exit code: {0:?}")]
	Run(ExitCode),

	#[error(transparent)]
	Io(#[from] io::Error),
}
