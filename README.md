## Overview

This is a simple POC that uses an api to get the cincinnati json graph data, inspect it and generate an imagesetconfiguration file for various openshift mirror tools. 

## POC 

This is still a WIP. I haven't completed all unit testing and there are bugs to sort out. 

I used a simple approach - Occam's razor

- A scientific and philosophical rule that entities should not be multiplied unnecessarily (KISS)
- Only RedHat's openshift release versions are supoprted

## Usage

Clone this repo

Execute the following to copy to local disk 

```bash
cd rust-release-introspection-tool

# create a directory called cache
mkdir cache && chmod 755 cache

make build 

# this will generate imagesetconfiguration files for v2 (oc-mirror) and v3 (rust-image-mirror) tools
./target/release/rust-release-introspection-tool --from-version 4.12.28 --to-version 4.14.16 --channel eus-4.14 --arch amd64 --loglevel debug 

```

## Unit Testing & Code coverage

Ensure grcov and  llvm tools-preview are installed

```
cargo install grcov 

rustup component add llvm-tools-preview

```

execute the tests

```
# add the -- --nocapture or --show-ouput flags to see println! statements
$ CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

# for individual tests
$ CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test create_diff_tar_pass -- --show-output
```

check the code coverage

```
$ grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "src/main.rs" -o target/coverage/html

```
