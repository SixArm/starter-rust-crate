# Starter Rust crate for creating applications

Starter provides our opinionated ways for creating Rust command line applications.


## Try it

Install:

```sh
cargo install starter
```

Run:

```sh
starter --test --verbose
```

Get the source code:

```sh
git clone https://github.com/sixarm/starter-rust-crate.git
```


## Goals

Goals include:

* Demonstrate how to combine crates for command line argument parsing, configuration, logging, errors, etc.
  
* Describe project conventions that help an application team with growth as well as long-term maintenance.

* Provide a starting point for our new projects.
  

## Anti-goals

Anti-goals include:

* Code everything from scratch. Instead, we want to use existing crates that are well-maintained and have a large community.

* Code freeze. Instead, this crate is a work in progress, and will evolve as we learn.

* Compatibility with no_std, or old Rust versions, or less-supported platforms. Instead, we want to use std, and recent Rust versions, and fully-supported platforms.


## Features

Features include:

* Run a command line argument parser, by using the crate [`clap`](https://crates.io/crates/clap).

* Load a configuration file, by using the crate [`confy`](https://crates.io/crates/confy).

* Create custom errors, by using the crate [`thiserror`](https://crates.io/crates/thiserror).

* Log messages such as trace and debug, by using the crate [`thiserror`](https://crates.io/crates/log).

* Log messages to configuration places, by using the crate [`env_logger`](https://crates.io/crates/env_logger).

* Use assert macros for testing and production, by using the crate [`assertables`](https://crates.io/crates/assertables)


## Project structure

The project structure aims for clean separation of concerns of generic application capabilities.

* `src` - Source code.
  * `app` - Application generic code.
    * `args` - Arguments structure; this may be set via `clap`.
    * `clap` - Command line argument parser i.e. `clap`.
    * `config` - Configuration structure; this may be set via `confy`.
    * `confy` - Configuration loader from a TOML file.
    * `mod` - Module file that includes all the modules.
    * `paths` - Path helpers, such as for directories and files.
    * `run` - Run function that handles everything; called by `main` or `lib`.
* `cspell.json` - CSpell is a spell checker for code.


## Cargo.toml

Dependencies:

* `assertables` - Assert macros for testing and runtime in production.
* `confy` - Configuration management.
* `clap` - Command Line Argument Parser.
* `env_logger` - A logger that can be configured via environment variables.
* `indoc` - Indented document literals.
* `map` The map macro for creating map collections.
* `log` - Lightweight logging facade.
* `serde` - Serialize/deserialize framework. Used by `confy`.
* `thiserror` - Error derive macro for the standard library std::error::Error trait.
