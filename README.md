# CLI Tool Built with Bun.js
This is a CLI tool built using Bun.js, designed for common file operations such as deleting, renaming, and optimizing files in a directory.

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
Delete files by specified index criteria (either even or odd).
---
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
