# Terminal Tool Kit - TermKit for the friends

[![npm version](https://badge.fury.io/js/term-toolkit.svg)](https://badge.fury.io/js/term-toolkit) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![npm](https://img.shields.io/npm/dt/term-toolkit)](https://www.npmjs.com/package/term-toolkit)

## Description
A CLI tool that offers several commands to help you with your daily tasks. The tool is built using Bun.js.

## Installation
To install the CLI tool, use the `(npm|pnpm|yarn|bun)` package manager:

```sh
npm install -g term-toolkit
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
Command: `term-toolkit deleter`

```sh
> term-toolkit deleter -h
Usage: term-toolkit deleter [options] <directory>

Delete files by n index, even or odd

Arguments:
  directory   Directory path

Options:
  -e, --even  Delete even indexes
  -h, --help  display help for command
```

Example:
```sh
term-toolkit deleter -e -d ./path/to/directory
```
This command will delete all files with even indexes in the directory.

Rename
---
Command: `term-toolkit rename`

```sh
> term-toolkit rename -h
Usage: term-toolkit rename [options] <newName>

Rename files by index, for example: file01.txt, file02.txt, ..., file10.txt

Arguments:
  newName                      New name for the files

Options:
  -d, --directory <directory>  Directory path (default: ".")
  -h, --help                   display help for command
```

Example:
```sh
term-toolkit rename -d ./path/to/directory "newName"
```
This command will rename all files in the directory with the new name and index.

Optimize
---
Command: `term-toolkit optimize`

```sh
> term-toolkit optimize -h
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

Example:
```sh
term-toolkit optimize -q 80 -o ./path/to/output/directory -d ./path/to/directory
```
This command will optimize all images in the directory with a quality of 80 and output the optimized images to the output directory.

Clone Repo
---
Command: `term-toolkit clone-repo`

```sh
> term-toolkit clone-repo -h
Usage: term-toolkit clone-repo [options] <repoUrl>

Clone a repository and optionally reset its history

Arguments:
  repoUrl                Repository URL

Options:
  --no-reset             Do not reset the history
  -o, --output <output>  Output folder
  -h, --help             display help for command
```

Example:
```sh
term-toolkit clone-repo --no-reset -o ./new-repo-folder
```
This command will clone the repository without resetting its history and output the cloned repository to the new-repo-folder.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
