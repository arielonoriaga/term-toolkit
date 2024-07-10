# Terminal Tool Kit - TermKit for the friends

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description
A CLI tool that offers several commands to help you with your daily tasks. The tool is built using Bun.js.

## Installation
To install the CLI tool, use the `(npm|pnpm|yarn|bun)` package manager:

```sh
npm install -g term-toolkit
```

Ensure you have [Bun.js](https://bun.sh/docs/installation) installed in your environment before running the tool.

```sh
bun install --global term-toolkit
```

#### General Usage

```sh
term-toolkit <command> [options]
```

```sh
Usage: term-toolkit [options] [command]

CLI Tools made in bun.js for common usage

Options:
  -V, --version                   output the version number
  -h, --help                      display help for command

Commands:
  deleter [options] <directory>   Delete files by n index, even or odd
  rename [options] <newName>      Rename files by index, for example: file01.txt, file02.txt, ..., file10.txt
  optimize [options] <directory>  Optimize images in a directory
  clone-repo [options] <repoUrl>  Clone a repository and optionally reset its history
  help [command]                  display help for command
```

Deleter
---
```sh
term-toolkit deleter -h
Usage: term-toolkit deleter [options] <directory>

Delete files by n index, even or odd

Arguments:
  directory   Directory path

Options:
  -e, --even  Delete even indexes
  -h, --help  display help for command
```

Rename
---
```sh
term-toolkit rename -h
Usage: term-toolkit rename [options] <newName>

Rename files by index, for example: file01.txt, file02.txt, ..., file10.txt

Arguments:
  newName                      New name for the files

Options:
  -d, --directory <directory>  Directory path (default: ".")
  -h, --help                   display help for command
```

Optimize
---
```
term-toolkit optimize -h
Usage: term-toolkit optimize [options] <directory>

Optimize images in a directory

Arguments:
  directory                Directory path with images or image

Options:
  -q, --quality <quality>  Quality of the image (default: "80")
  -o, --output <output>    Output directory
  --keep-original          Keep the original image (default: true)
  -h, --help               display help for command
```

Clone Repo
---
```sh
term-toolkit clone-repo -h
Usage: term-toolkit clone-repo [options] <repoUrl>

Clone a repository and optionally reset its history

Arguments:
  repoUrl                Repository URL

Options:
  --no-reset             Do not reset the history
  -o, --output <output>  Output folder
  -h, --help             display help for command
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
