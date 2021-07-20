//!
//! YUL to LLVM compiler binary.
//!

pub mod arguments;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() {
    std::process::exit(match main_inner() {
        Ok(()) => compiler_const::exit_code::SUCCESS,
        Err(error) => {
            eprintln!("{:?}", error);
            compiler_const::exit_code::FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> Result<(), yul_compiler::Error> {
    let arguments = Arguments::new();

    let target = yul_compiler::Target::try_from(arguments.target.as_str())
        .map_err(yul_compiler::Error::Target)?;

    let code = if arguments.input.to_string_lossy() == "-" {
        let mut buffer = String::with_capacity(16384);
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        std::fs::read_to_string(&arguments.input)?
    };

    let representation = yul_compiler::compile(
        &code,
        target,
        arguments.optimization_level,
        arguments.dump_llvm,
    )?;

    let text = representation.clone().into_bytes();
    let binary = zkevm_assembly::Assembly::try_from(representation)?;
    let binary = Vec::<u8>::from(&binary);

    let text_file_name = match target {
        yul_compiler::Target::LLVM => compiler_const::file_name::LLVM_SOURCE,
        yul_compiler::Target::zkEVM => compiler_const::file_name::ZKEVM_ASSEMBLY,
    };
    let text_file_extension = match target {
        yul_compiler::Target::LLVM => compiler_const::extension::LLVM_SOURCE,
        yul_compiler::Target::zkEVM => compiler_const::extension::ZKEVM_ASSEMBLY,
    };
    let text_file_path = PathBuf::from(format!("{}.{}", text_file_name, text_file_extension,));
    File::create(&text_file_path)
        .expect("Text file creating error")
        .write_all(text.as_slice())
        .expect("Text file writing error");

    let binary_file_path = PathBuf::from(compiler_const::file_name::ZKEVM_BINARY);
    File::create(&binary_file_path)
        .expect("Binary file creating error")
        .write_all(binary.as_slice())
        .expect("Binary file writing error");

    Ok(())
}
