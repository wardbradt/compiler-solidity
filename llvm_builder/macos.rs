//!
//! The LLVM `macos` build script.
//!

use std::process::Command;

///
/// The building sequence.
///
pub fn build() -> anyhow::Result<()> {
    crate::utils::command(
        Command::new("cmake").args(&[
            "-S",
            "./compiler-llvm/llvm/",
            "-B",
            "./compiler-llvm/build/",
            "-G",
            "Ninja",
            "-DCMAKE_INSTALL_PREFIX='./llvm_build/'",
            "-DCMAKE_BUILD_TYPE='Release'",
            "-DLLVM_TARGETS_TO_BUILD='SyncVM'",
            "-DLLVM_OPTIMIZED_TABLEGEN='On'",
            "-DLLVM_BUILD_TESTS='Off'",
            "-DLLVM_BUILD_DOCS='Off'",
            "-DLLVM_INCLUDE_DOCS='Off'",
            "-DLLVM_INCLUDE_TESTS='Off'",
            "-DLLVM_ENABLE_ASSERTIONS='Off'",
            "-DLLVM_ENABLE_TERMINFO='Off'",
            "-DLLVM_ENABLE_DOXYGEN='Off'",
            "-DLLVM_ENABLE_SPHINX='Off'",
            "-DLLVM_ENABLE_OCAMLDOC='Off'",
            "-DLLVM_ENABLE_BINDINGS='Off'",
        ]),
        "LLVM building cmake",
    )?;

    crate::utils::command(
        Command::new("ninja").args(&["-C", "./compiler-llvm/build/", "install"]),
        "LLVM building ninja",
    )?;

    Ok(())
}
