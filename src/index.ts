#!/usr/bin/env bun

import { program } from 'commander';

program
  .version('1.0.0')
  .description('My Custom CLI Tool')
  .option('-n, --name <type>', 'specify your name')
  .command('greet')
  .description('Greet the user')
  .action((options) => {
    console.log('options', options)
    const name = program.opts().name || 'friend';
    console.log(`Hello, ${name}!`);
  });

program.parse(process.argv);
