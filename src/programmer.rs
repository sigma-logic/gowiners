use std::{
	ffi::OsStr,
	io,
	path::PathBuf,
	process::{Command, ExitCode, Stdio},
	str::FromStr,
};

use thiserror::Error;

#[derive(Debug)]
pub struct Programmer {
	bin: PathBuf,
	pub device_family: String,
	pub op: Op,
	pub fs_file: Option<PathBuf>,
	pub cable: Option<String>,
	pub frequency: Option<String>,
}

impl Programmer {
	pub fn new(bin: impl Into<PathBuf>, device_family: String, op: Op) -> Self {
		Self {
			bin: bin.into(),
			device_family,
			op,
			fs_file: None,
			cable: None,
			frequency: None,
		}
	}

	pub fn run(&mut self) -> Result<(), ProgrammerError> {
		let mut prg = Command::new(&self.bin);

		prg.stdout(Stdio::inherit()).stderr(Stdio::inherit());

		// --device
		prg.args(["--device", &self.device_family]);

		// --run
		prg.args(["--run", &(self.op as u8).to_string()]);

		// --fsFile
		if let Some(bitstream) = &self.fs_file {
			prg.args([
				OsStr::new("--fsFile"),
				bitstream.canonicalize()?.as_os_str(),
			]);
		}

		if let Some(cable) = &self.cable {
			prg.args(["--cable", cable]);
		}

		if let Some(frequency) = &self.frequency {
			prg.args(["--frequency", frequency]);
		}

		Ok(())
	}
}

#[derive(Error, Debug)]
pub enum ProgrammerError {
	#[error("Failed to spawn pipeline shell")]
	Spawn(#[source] io::Error),

	#[error("Pipeline run failed with exit code: {0:?}")]
	Run(ExitCode),

	#[error(transparent)]
	Io(#[from] io::Error),
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Op {
	#[default]
	ReadDeviceCodes = 0,
	Reprogram = 1,
	SRAMProgram = 2,
	SRAMRead = 3,
	SRAMProgramAndVerify = 4,
	EmbFlashEraseProgram = 5,
	EmbFlashEraseProgramVerify = 6,
	EmbFlashEraseOnly = 7,
	ExFlashEraseProgram = 8,
	ExFlashEraseProgramVerify = 9,
	ExFlashBulkErase = 10,
	ExFlashVerify = 11,
	ExFlashEraseProgramInBscan = 12,
	ExFlashEraseProgramVerifyInBscan = 13,
	ExFlashBulkEraseInBscan = 14,
	ExFlashVerifyInBscan = 15,
	SRAMProgramJtag1149 = 16,
	SRAMProgramVerifyJtag1149 = 17,
	BsdlRead = 18,
	EmbFlash2ndEraseProgram = 19,
	EmbFlash2ndEraseProgramVerify = 20,
	EmbFlash2ndEraseOnly = 21,
	ConnectToJtagOfMCU = 23,
	SRAMErase = 24,
	AuthenticationCodeEraseProgramVerify = 25,
	AuthenticationCodeRead = 26,
	FirmwareEraseProgramSecurely = 27,
	FirmwareEraseOnly = 28,
	FirmwareEraseProgram = 29,
	FirmwareEraseProgramVerify = 30,
	ExFlashCBinEraseProgram = 31,
	ExFlashCBinEraseProgramVerify = 32,
	MFGWriteIRef = 34,
	CSRFileEraseProgramVerify = 35,
	ExFlashEraseProgramThruGaoBridge = 36,
	ExFlashEraseProgramVerifyThruGaoBridge = 37,
	ExFlashCBinEraseProgramThruGaoBridge = 38,
	ExFlashCBinEraseProgramVerifyThruGaoBridge = 39,
	#[allow(non_camel_case_types)]
	DKGoAI_GW1NSR4C_QN48_V1_1 = 40,
	#[allow(non_camel_case_types)]
	DKGoAI_GW1NSR4C_QN48_V2_2 = 41,
	#[allow(non_camel_case_types)]
	DKGoAI_GW2AR18_QN88P_V1_1 = 42,
	SFlashEraseProgram = 44,
	SFlashEraseProgramVerify = 45,
	SFlashVerifyOnly = 46,
	SFlashBulkErase = 47,
	SFlashBackgroundEraseProgram = 48,
	SFlashBackgroundEraseProgramVerify = 49,
	SFlashEraseProgramVerifyThruGaoBridge = 50,
	ExFlashDetectID = 51,
	ExFlashBulkErase5A = 52,
	ExFlashEraseProgram5A = 53,
	ExFlashEraseProgramVerify5A = 54,
	ExFlashCBinEraseProgram5A = 55,
	ExFlashCBinEraseProgramVerify5A = 56,
	I2CProgramSRAM = 57,
	I2CProgramFlash = 58,
	I2CEraseFlashOnly = 59,
	I2CEraseFlashOnlyThruI2CSPI = 60,
	I2CEraseProgramFlashThruI2CSPI = 61,
	EBRRead = 62,
	SFlashBackgroundEraseProgramVerifyThruGaoBridge = 63,
	SFlashBulkEraseInBscan = 64,
	SFlashEraseProgramInBscan = 65,
	ExFlashVerify5A = 66,
	ExFlashVerifyThruGaoBridge5A = 67,
	ExFlashEraseProgramThruGaoBridge5A = 68,
	ExFlashEraseProgramVerifyThruGaoBridge5A = 69,
	EmbFlashBackgroundEraseProgram = 70,
	EmbFlashBackgroundEraseProgramVerify = 71,
	EmbFlashBackgroundEraseOnly = 72,
	ReadUserCode = 73,
	ReadStatusRegister = 74,
	SetFlashQEFor9x18x = 75,
	SetExFlashQEForGW5AT = 76,
	ExFlashEraseProgramVerifyThruUARTIPSPI = 77,
	SRAMReprogramThruUARTIPSPI = 78,
}

impl FromStr for Op {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Read Device Codes" => Ok(Op::ReadDeviceCodes),
			"Reprogram" => Ok(Op::Reprogram),
			"SRAM Program" => Ok(Op::SRAMProgram),
			"SRAM Read" => Ok(Op::SRAMRead),
			"SRAM Program and Verify" => Ok(Op::SRAMProgramAndVerify),
			"embFlash Erase,Program" => Ok(Op::EmbFlashEraseProgram),
			"embFlash Erase,Program,Verify" => Ok(Op::EmbFlashEraseProgramVerify),
			"embFlash Erase Only" => Ok(Op::EmbFlashEraseOnly),
			"exFlash Erase,Program" => Ok(Op::ExFlashEraseProgram),
			"exFlash Erase,Program,Verify" => Ok(Op::ExFlashEraseProgramVerify),
			"exFlash Bulk Erase" => Ok(Op::ExFlashBulkErase),
			"exFlash Verify" => Ok(Op::ExFlashVerify),
			"exFlash Erase,Program in bscan" => Ok(Op::ExFlashEraseProgramInBscan),
			"exFlash Erase,Program,Verify in bscan" => {
				Ok(Op::ExFlashEraseProgramVerifyInBscan)
			}
			"exFlash Bulk Erase in bscan" => Ok(Op::ExFlashBulkEraseInBscan),
			"exFlash Verify in bscan" => Ok(Op::ExFlashVerifyInBscan),
			"SRAM Program JTAG 1149" => Ok(Op::SRAMProgramJtag1149),
			"SRAM Program,Verify JTAG 1149" => Ok(Op::SRAMProgramVerifyJtag1149),
			"bsdl read" => Ok(Op::BsdlRead),
			"embFlash 2nd Erase,Program" => Ok(Op::EmbFlash2ndEraseProgram),
			"embFlash 2nd Erase,Program,Verify" => {
				Ok(Op::EmbFlash2ndEraseProgramVerify)
			}
			"embFlash 2nd Erase Only" => Ok(Op::EmbFlash2ndEraseOnly),
			"Connect to JTAG of MCU" => Ok(Op::ConnectToJtagOfMCU),
			"SRAM Erase" => Ok(Op::SRAMErase),
			"Authentication Code Erase,Program,Verify" => {
				Ok(Op::AuthenticationCodeEraseProgramVerify)
			}
			"Authentication Code Read" => Ok(Op::AuthenticationCodeRead),
			"Firmware Erase,Program Securely" => Ok(Op::FirmwareEraseProgramSecurely),
			"Firmware Erase Only" => Ok(Op::FirmwareEraseOnly),
			"Firmware Erase,Program" => Ok(Op::FirmwareEraseProgram),
			"Firmware Erase,Program,Verify" => Ok(Op::FirmwareEraseProgramVerify),
			"exFlash C Bin Erase,Program" => Ok(Op::ExFlashCBinEraseProgram),
			"exFlash C Bin Erase,Program,Verify" => {
				Ok(Op::ExFlashCBinEraseProgramVerify)
			}
			"MFG Write iRef" => Ok(Op::MFGWriteIRef),
			"CSR File Erase,Program,Verify" => Ok(Op::CSRFileEraseProgramVerify),
			"exFlash Erase,Program thru GAO-Bridge" => {
				Ok(Op::ExFlashEraseProgramThruGaoBridge)
			}
			"exFlash Erase,Program,Verify thru GAO-Bridge" => {
				Ok(Op::ExFlashEraseProgramVerifyThruGaoBridge)
			}
			"exFlash C Bin Erase,Program thru GAO-Bridge" => {
				Ok(Op::ExFlashCBinEraseProgramThruGaoBridge)
			}
			"exFlash C Bin Erase,Program,Verify thru GAO-Bridge" => {
				Ok(Op::ExFlashCBinEraseProgramVerifyThruGaoBridge)
			}
			"sFlash Erase,Program" => Ok(Op::SFlashEraseProgram),
			"sFlash Erase,Program,Verify" => Ok(Op::SFlashEraseProgramVerify),
			"sFlash Verify Only" => Ok(Op::SFlashVerifyOnly),
			"sFlash Bulk Erase" => Ok(Op::SFlashBulkErase),
			"sFlash Background Erase,Program" => Ok(Op::SFlashBackgroundEraseProgram),
			"sFlash Background Erase,Program,Verify" => {
				Ok(Op::SFlashBackgroundEraseProgramVerify)
			}
			"sFlash Erase,Program,Verify thru GAO-Bridge" => {
				Ok(Op::SFlashEraseProgramVerifyThruGaoBridge)
			}
			"exFlash Detect ID" => Ok(Op::ExFlashDetectID),
			"exFlash Bulk Erase 5A" => Ok(Op::ExFlashBulkErase5A),
			"exFlash Erase,Program 5A" => Ok(Op::ExFlashEraseProgram5A),
			"exFlash Erase,Program,Verify 5A" => Ok(Op::ExFlashEraseProgramVerify5A),
			"exFlash C Bin Erase,Program 5A" => Ok(Op::ExFlashCBinEraseProgram5A),
			"exFlash C Bin Erase,Program,Verify 5A" => {
				Ok(Op::ExFlashCBinEraseProgramVerify5A)
			}
			"I2C Program SRAM" => Ok(Op::I2CProgramSRAM),
			"I2C Program Flash" => Ok(Op::I2CProgramFlash),
			"I2C Erase Flash Only" => Ok(Op::I2CEraseFlashOnly),
			"I2C Erase Flash Only thru I2C-SPI" => {
				Ok(Op::I2CEraseFlashOnlyThruI2CSPI)
			}
			"I2C Erase,Program Flash thru I2C-SPI" => {
				Ok(Op::I2CEraseProgramFlashThruI2CSPI)
			}
			"EBR Read" => Ok(Op::EBRRead),
			"sFlash Background Erase,Program,Verify thru GAO-Bridge" => {
				Ok(Op::SFlashBackgroundEraseProgramVerifyThruGaoBridge)
			}
			"sFlash Bulk Erase in bscan" => Ok(Op::SFlashBulkEraseInBscan),
			"sFlash Erase,Program in bscan" => Ok(Op::SFlashEraseProgramInBscan),
			"exFlash Verify 5A" => Ok(Op::ExFlashVerify5A),
			"exFlash Verify thru GAO-Bridge 5A" => {
				Ok(Op::ExFlashVerifyThruGaoBridge5A)
			}
			"exFlash Erase,Program thru GAO-Bridge 5A" => {
				Ok(Op::ExFlashEraseProgramThruGaoBridge5A)
			}
			"exFlash Erase,Program,Verify thru GAO-Bridge 5A" => {
				Ok(Op::ExFlashEraseProgramVerifyThruGaoBridge5A)
			}
			"embFlash Background Erase,Program" => {
				Ok(Op::EmbFlashBackgroundEraseProgram)
			}
			"embFlash Background Erase,Program,Verify" => {
				Ok(Op::EmbFlashBackgroundEraseProgramVerify)
			}
			"embFlash Background Erase Only" => Ok(Op::EmbFlashBackgroundEraseOnly),
			"Read User Code" => Ok(Op::ReadUserCode),
			"Read Status Register" => Ok(Op::ReadStatusRegister),
			"Set Flash QE For 9x/18x" => Ok(Op::SetFlashQEFor9x18x),
			"Set ExFlash QE For GW5A(T)" => Ok(Op::SetExFlashQEForGW5AT),
			"exFlash Erase,Program,Verify thru UART-IP-SPI" => {
				Ok(Op::ExFlashEraseProgramVerifyThruUARTIPSPI)
			}
			"SRAM Reprogram thru UART-IP-SPI" => Ok(Op::SRAMReprogramThruUARTIPSPI),
			_ => Err(format!("Unknown operation: {}", s)),
		}
	}
}
