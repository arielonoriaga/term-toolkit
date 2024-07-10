# Terminal Tool Kit - TermKit for the friends

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description
A CLI tool that offers several commands to help you with your daily tasks. The tool is built using Bun.js.

## Installation
To install the CLI tool, use the `(npm|pnpm|yarn|bun)` package manager:

```sh
npm install -g termkit
```

Ensure you have [Bun.js](https://bun.sh/docs/installation) installed in your environment before running the tool.

```sh
bun install --global termkit
```

#### General Usage

```sh
term-toolkit <command> [options]
```

##### Commands
 - deleter: Delete files by specified index criteria (either even or odd).
 - renamer: Rename files by specified index criteria (either even or odd).
 - optimizer: Optimize files by specified index criteria (either even or odd).
 - clone-repo: Clone a repository from a specified URL.

Deleter
---
Delete files by specified index criteria (either even or odd).
```sh
termkit deleter [options]
```
Arguments:

<directory>: Directory path where files will be deleted.
Options:

-e, --even: Delete files with even indexes. If not specified, odd indexes will be used.

Renamer
---
Rename files by specified index criteria (either even or odd).
```sh
termkit renamer [options]
```
Rename files in a sequence, for example: file01.txt, file02.txt, ..., file10.txt.


## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
