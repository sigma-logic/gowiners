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
pub enum Bitstream {
	Fs(PathBuf)
}

#[derive(Debug, Deserialize)]
pub struct ProgrammerPreset {
	#[serde(flatten)]
	pub args: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammerConfig {
	#[serde(flatten)]
	pub args: HashMap<String, String>,
	pub preset: HashMap<String, ProgrammerPreset>
}

#[derive(Debug, Deserialize)]
pub struct Project {
	pub name: String,
	pub version: u32,
	pub device: Device,
	pub hdl: Hdl,
	pub files: HashMap<String, MaybeList<PathBuf>>,
	pub programmer: ProgrammerConfig,
}
