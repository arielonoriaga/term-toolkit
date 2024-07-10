#!/usr/bin/env bun

import { program } from 'commander';
import {getPackageJson} from './scripts/utils/getPackageInfo';

const packageJsonFile = getPackageJson()

program
  .version(packageJsonFile.version)
  .description('CLI Tools made in bun.js for common usage')

program
  .command('deleter')
  .description('Delete files by n index, even or odd')
  .argument('<directory>', 'Directory path')
  .option('-e, --even', 'Delete even indexes', 'odd')
  .action((directory, options) => {
    import('./scripts/deleter').then(({ deleter }) => {
      deleter(directory, options.even);
    });
  })

program
  .command('rename')
  .description('Rename files by index, for example: file01.txt, file02.txt, ..., file10.txt')
  .argument('<directory>', 'Directory path')
  .argument('<newName>', 'New name for the files')
  .action((directory, newName) => {
    import('./scripts/rename-sequence').then(({ renameFiles }) => {
      renameFiles(directory, newName);
    });
  })

program
  .command('optimize')
  .description('Optimize images in a directory')
  .argument('<directory>', 'Directory path')
  .option('-q, --quality <quality>', 'Quality of the image', '80')
  .option('-o, --output <output>', 'Output directory')
  .option('--keep-original', 'Keep the original image', true)
  .action((directory, options) => {
    import('./scripts/optimizer').then(({ optimize }) => {
      optimize({
        inputPath: directory,
        outputPath: options.output,
        quality: options.quality,
        keepOriginal: options.keepOriginal
      });
    });
  })

program.parse(process.argv);
