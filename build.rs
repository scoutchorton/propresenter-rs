use std::{
	env::var,
	fs::{self, create_dir, OpenOptions},
	io::{BufRead, BufReader, Error as IOError, Write},
	path::PathBuf,
};

use glob::{glob, GlobError, PatternError};
use protobuf_codegen;
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

/// Clean up protoc generated Rust files
fn clean_files(root: PathBuf) -> Result<(), ProtobufBuildError> {
	// Get all Rust files in the build directory
	let root = root.join("*.rs");
	let source_files: Vec<_> = glob(root.to_str().unwrap())?
		.into_iter()
		.flatten()
		.collect()
		;

	for path in source_files {
		clean_file(path)?;
	}

	Ok(())
}
/// Clean up a protoc generated Rust file so that it compiles properly
fn clean_file(path: PathBuf) -> Result<(), ProtobufBuildError> {
	// Yoinked this solution: https://stackoverflow.com/a/62763629

	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.open(&path)?
		;
	let lines = BufReader::new(&file).lines().flatten();

	// Skip prelude lines that start with comments or are empty
	let mut lines = lines.skip_while(|line| line.len() == 0 || line[..3] == String::from("// ")).peekable();

	// Pull in beginning without comments
	let mut beginning = Vec::new();
	while let Some(line) = lines.next_if(|l| l.len() == 0 || l[..3] == String::from("#![")) {
		beginning.push(line);
	}
	let beginning = beginning.into_iter().map(|line| line.replacen("#!", "#", 1));
	let lines = lines.skip(2);

	// Merge allow items and rust of code
	let lines = beginning.into_iter().chain(lines);

	// Write final output
	fs::write(path, lines.collect::<Vec<_>>().join("\n").as_bytes())?;

	//Err(ProtobufBuildError::FileNotFound)
	Ok(())
}

/// Convert protobuf files to Rust source code
fn build_proto() -> Result<(), ProtobufBuildError> {
	let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let proto_src_dir = root_dir.join("ProPresenter7-Proto/Proto7.16.2");
	let proto_src_files = proto_src_dir.join("*.proto");
	let proto_out_dir = PathBuf::from(var("OUT_DIR").unwrap()).join("proto");
	let proto_out_module = PathBuf::from(var("OUT_DIR").unwrap()).join("mod.rs");

	// @todo check that the protobuf folder exists (if not show a warning with instructions to pull submodule)

	// Create output directory
	if !proto_out_dir.exists() {
		let _ = create_dir(&proto_out_dir)?;
	}

	// Get list of files, excluding files that generate errors
	let proto_files: Vec<_> = glob(proto_src_files.to_str().unwrap())?
		.into_iter()
		.flatten()
		.collect()
		;

	// Convert all files into Rust source code
	protobuf_codegen::Codegen::new()
		// Use vendored protoc compiler package
		.protoc().protoc_path(protoc_bin_vendored::protoc_bin_path().unwrap().as_path())
		.include(&proto_src_dir.to_str().unwrap())
		.inputs(&proto_files)
		.out_dir(&proto_out_dir)
		.run_from_script()
		;

	// Clean up generated files\
	let _ = clean_files(proto_out_dir)?;

	// Create module file
	let mut module = OpenOptions::new()
		.create(true)
		.write(true)
		.open(proto_out_module)
		.unwrap();
	for path in proto_files {
		let base = match path.file_stem() {
			Some(p) => p.to_str().unwrap(),
			_ => continue,
		};

		let _ = write!(&mut module, "pub mod {} {{ " /* "}}" */, base).unwrap();
		let _ = write!(&mut module, "include!(concat!(env!(\"OUT_DIR\"), \"/proto/{}.rs\"));", base);
		let _ = writeln!(&mut module, " }}");
	}

	println!("cargo:rerun-if-changed=build.rs");

	Ok(())
}

fn main() {
	let _ = build_proto();
}