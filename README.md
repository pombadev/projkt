<div align="center">

# `projkt`

**Generate project related stuffs such as .gitignore, license, readmes and more**

[![crates.io](https://img.shields.io/crates/v/projkt.svg?style=flat-square)](https://crates.io/crates/projkt)
[![github.com](https://img.shields.io/github/license/pjmp/projkt?style=flat-square)](https://github.com/pjmp/projkt)
[![github.com](https://img.shields.io/badge/contributions-welcome-brightgreen?style=flat-square)](https://github.com/pjmp/projkt)
</div>

## What and why?

> `projkt` is a commandline tool to generate boring stuffs such as `.gitignore`, `README`, `license` and more quickly so that you can send more time in your projects rather than skeleton files.

There are many projects that does one of these things but I haven't found any that combines all of them.

## Features

- Fuzzy selector
- Multi selection
- Preview window
- Generate gitignore, license, readme etc

## Demo

![demo](https://i.imgur.com/AZwwAnR.gif)

## Usage

```shell

projkt <generator> # opens a fuzzy file picker

projkt <options> <generator> <args> # write output to file

# generate `.gitignore` file(s)
projkt gitignore

projkt --overwrite gitignore rust

# generate license file(s)
projkt --dest=~/Codes/repo license rust

```

## Installation

- From *crates.io*

```shell
cargo install projkt
```

- From *source*
```shell
git clone https://github.com/pjmp/projkt.git

cd projkt

cargo run
```

## Available generators

- gitignore
- license
- readme
- *your suggestions*

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/) or [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
