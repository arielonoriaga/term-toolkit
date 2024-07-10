#!/usr/bin/env bun

import { program } from 'commander';
import {getPackageJson} from './src/scripts/utils/getPackageInfo';

const packageJsonFile = getPackageJson()

program
  .version(packageJsonFile.version)
  .description('CLI Tools made in bun.js for common usage')

program
  .command('deleter')
  .description('Delete files by n index, even or odd, by default it will delete even indexes')
  .argument('<directory>', 'Directory path')
  .action((directory) => {
    import('./src/scripts/deleter').then(({ listAndDeleteFiles }) => {
      listAndDeleteFiles(directory);
    });
  })

program
  .command('rename')
  .description('Rename files by index, for example: file01.txt, file02.txt, ..., file10.txt')
  .argument('<directory>', 'Directory path')
  .argument('<newName>', 'New name for the files')
  .action((directory, newName) => {
    import('./src/scripts/rename-sequence').then(({ renameFiles }) => {
      renameFiles(directory, newName);
    });
  })

program.parse(process.argv);
