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
	pub standard: String,
	pub include: Option<Vec<PathBuf>>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaybeList<T> {
	Single(T),
	List(Vec<T>),
}

impl<T: Clone> MaybeList<T> {
	pub fn to_vec(&self) -> Vec<T> {
		match self {
			MaybeList::Single(it) => vec![it.clone()],
			MaybeList::List(it) => it.clone(),
		}
	}
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
pub struct PlaceAndRouteConfig {
	pub place_mode: Option<u32>,
	pub route_mode: Option<u32>,
	pub replicate: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct BitstreamConfig {
	pub compress: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct Project {
	pub name: String,
	pub version: u32,
	pub device: Device,
	pub hdl: Hdl,
	pub files: HashMap<String, MaybeList<String>>,
	pub pnr: Option<PlaceAndRouteConfig>,
	pub bitstream: Option<BitstreamConfig>,
	pub programmer: ProgrammerConfig,
}
