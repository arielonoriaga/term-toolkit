# CLI Tool Built with Bun.js

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description
The CLI tool offers several commands, each with its own description and options. The commands include:
- deleter: Delete files by specified index criteria (either even or odd).
- renamer: Rename files by specified index criteria (either even or odd).
- optimizer: Optimize files by specified index criteria (either even or odd).

## Installation
To install the CLI tool, use the npm package manager:

```sh
npm install -g your-cli-tool
```

Ensure you have Bun.js installed in your environment before running the tool.

### Usage
The CLI tool offers several commands, each with its own description and options.


#### General Usage

```sh
your-cli-tool <command> [options]
```

##### Commands
 - deleter: Delete files by specified index criteria (either even or odd).
 - renamer: Rename files by specified index criteria (either even or odd).
 - optimizer: Optimize files by specified index criteria (either even or odd).

Deleter
---
Delete files by specified index criteria (either even or odd).
```sh
your-cli-tool deleter [options]
```
Arguments:

<directory>: Directory path where files will be deleted.
Options:

-e, --even: Delete files with even indexes. If not specified, odd indexes will be used.

Renamer
---
Rename files by specified index criteria (either even or odd).
```sh
your-cli-tool renamer [options]
```
Rename files in a sequence, for example: file01.txt, file02.txt, ..., file10.txt.


## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
