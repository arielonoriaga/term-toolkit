#!/usr/bin/env bun
import { execSync } from 'child_process'
import { basename } from 'path'

interface Args {
  repoUrl: string
  resetHistory?: boolean
  outputFolder?: string
}

const _cloneRepo = ({ repoUrl, resetHistory = true, outputFolder }: Args) => {
  try {
    const repoName = basename(repoUrl, '.git')
    const targetFolder = outputFolder || repoName

    execSync(`git clone ${repoUrl} ${targetFolder}`, { stdio: 'inherit' })

    if (!resetHistory) return
    process.chdir(`./${targetFolder}`)

    execSync('rm -rf .git', { stdio: 'inherit' })
    execSync('git init', { stdio: 'inherit' })
    execSync('git add .', { stdio: 'inherit' })
    execSync('git commit -m "Initial commit"', { stdio: 'inherit' })
  } catch (error) {
    console.error(error)
  }
}

export const cloneRepo = (args: Args) => {
  if (!args.repoUrl) {
    console.error('Please provide a repository URL')
    process.exit(1)
  }

  _cloneRepo(args)
}
