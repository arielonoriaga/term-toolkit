#!/usr/bin/env bun
import { execSync } from 'child_process';
import { basename } from 'path';

type Args = {
  repoUrl: string;
  resetHistory?: boolean;
  outputFolder?: string;
};

const _cloneRepo = ({ repoUrl, resetHistory = true, outputFolder }: Args) => {
  try {
    const repoName = basename(repoUrl, '.git');
    const targetFolder = outputFolder || repoName;

    execSync(`git clone ${repoUrl} ${targetFolder}`, { stdio: 'inherit' });
    console.log(`Repository cloned to ./${targetFolder}`);

    if (!resetHistory) return
    process.chdir(`./${targetFolder}`);

    execSync('rm -rf .git', { stdio: 'inherit' });
    console.log('Removed .git directory');

    execSync('git init', { stdio: 'inherit' });
    console.log('Initialized new git repository');

    execSync('git add .', { stdio: 'inherit' });
    console.log('Added all files to the new git repository');

    execSync('git commit -m "Initial commit"', { stdio: 'inherit' });
    console.log('Committed the changes');
  } catch (error) {
    console.error(error);
  }
}

export const cloneRepo = (args: Args) => {
  if (!args.repoUrl) {
    console.error('Please provide a repository URL');
    process.exit(1);
  }

  _cloneRepo(args);
}
