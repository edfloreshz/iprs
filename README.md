<div align="center">
  <img src="https://raw.githubusercontent.com/edfloreshz/iprs/master/assets/logo.png" width="150" />

  <h1>InterPlanetary Replication System</h1>
</div>

![Rust](https://github.com/edfloreshz/iprs/workflows/Rust/badge.svg?branch=master)
[![Run on Repl.it](https://repl.it/badge/github/edfloreshz/iprs)](https://repl.it/github/edfloreshz/iprs)

The InterPlanetary Sync System or IPRS is a tool that allows users to store and sync their files in nearby nodes connected to the IPFS Network. 


## How does it work?

**IPRS** will run background as a daemon on your personal computer and keep track of every change you make to your file system on supported folders, it will upload every new version of your files and folders to nearby nodes connected to the IPFS network and keep them up to date, you will be able to access them anywhere. Everything you upload will always be available to you.

## Installing from release
Download a build from the [release](https://github.com/edfloreshz/iprs/releases) page and unzip it.

To install run `./install.sh`

To uninstall run `./uninstall.sh`

## Installing with cargo
Install the binary with `cargo install --path .`

## Run the project
Run the binary with `iprs`

Run the program it with `cargo run`

Build it with `cargo build`

Test it with `cargo test`
