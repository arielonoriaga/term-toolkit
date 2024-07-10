import { readFileSync } from 'fs'
import { join } from 'path'

interface PackageJson {
  version: string
}

export const getPackageJson = (): PackageJson => {
  const packageJsonPath = join(__dirname, '../../../package.json')
  const packageJsonContent = readFileSync(packageJsonPath, 'utf-8')

  return JSON.parse(packageJsonContent)
}
