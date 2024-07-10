import { readdir, rename } from 'fs/promises'
import { join, extname, isAbsolute } from 'path'
import { homedir } from 'os'

const _renameFiles = async  (directory: string, baseName: string): Promise<void> => {
  try {
    const files: string[] = await readdir(directory)
    const sortedFiles = files.sort()

    const totalFiles = files.length
    const padLength = totalFiles.toString().length

    for (let i = 0; i < totalFiles; i++) {
      const file = sortedFiles[i]
      const fileExtension = extname(file)
      const paddedNumber = i.toString().padStart(padLength, '0')
      const newFileName = `${baseName}${paddedNumber}${fileExtension}`
      const oldFilePath = join(directory, file)
      const newFilePath = join(directory, newFileName)

      try {
        await rename(oldFilePath, newFilePath)
        console.log(`Renamed ${oldFilePath} to ${newFilePath}`)
      } catch (err) {
        console.error(`Error renaming file ${oldFilePath}:`, err)
      }
    }
  } catch (err) {
    console.error(`Error reading directory ${directory}:`, err)
  }
}

export const renameFiles = async (directory: string, baseName: string): Promise<void> => {
  if (!directory) {
    console.error('Please provide a directory path.')
    process.exit(1)
  }

  if (!baseName) {
    console.error('Please provide a base name.')
    process.exit(1)
  }

  if (directory.startsWith('~')) {
    directory = join(homedir(), directory.slice(1))
  }

  if (!isAbsolute(directory)) {
    directory = join(process.cwd(), directory)
  }

  _renameFiles(directory, baseName)
}
