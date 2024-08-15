use std::{
	env::self,
	io::Error as IOError,
	path::PathBuf,
};

use glob::{glob, GlobError, PatternError};
use prost_build;
use protoc_bin_vendored;

enum ProtobufBuildError {
	IOError,
	FileNotFound,
	GlobError,
}
impl From<IOError> for ProtobufBuildError {
	fn from(_value: IOError) -> Self {
		Self::IOError
	}
}
impl From<PatternError> for ProtobufBuildError {
	fn from(_value: PatternError) -> Self {
		Self::FileNotFound
	}
}
impl From<GlobError> for ProtobufBuildError {
	fn from(_value: GlobError) -> Self {
		Self::GlobError
	}
}

/// Convert protobuf files to Rust source code
fn build_proto() -> Result<(), ProtobufBuildError> {
	let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let proto_src_dir = root_dir.join("ProPresenter7-Proto").join("Proto7.16.2");
	let proto_src_files = proto_src_dir.join("*.proto");

	// Get list of files, excluding files that generate errors
	let proto_files: Vec<_> = glob(proto_src_files.to_str().unwrap())?
		.into_iter()
		.flatten()
		.collect()
		;

	// Convert all files into Rust source code
	env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap().as_path());
	prost_build::compile_protos(
		proto_files.as_slice(),
		&[proto_src_dir.to_str().unwrap()]
	)?;

	println!("cargo:rerun-if-changed=build.rs");

	Ok(())
}

fn main() {
	let _ = build_proto();
}