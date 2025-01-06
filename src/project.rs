use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Device {
	pub family: String,
	pub part: String,
}

#[derive(Debug, Deserialize)]
pub struct Hdl {
	pub top: Option<String>,
	pub include: Vec<PathBuf>,
	pub standard: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaybeList<T> {
	Single(T),
	List(Vec<T>),
}

#[derive(Debug, Deserialize)]
pub struct ProgrammerPreset {
	pub op: Option<String>,
	pub cable: Option<String>,
	pub frequency: Option<String>,
	pub bitstream: Option<String>,

	#[serde(flatten)]
	pub overrides: HashMap<String, ProgrammerPreset>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
	pub name: String,
	pub version: u32,
	pub device: Device,
	pub hdl: Hdl,
	pub files: HashMap<String, MaybeList<PathBuf>>,
	pub programmer: ProgrammerPreset,
}
