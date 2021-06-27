
rustup component add llvm-tools-preview
export RUSTFLAGS="-Zinstrument-coverage"
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
