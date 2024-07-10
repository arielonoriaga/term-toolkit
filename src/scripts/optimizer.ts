import { readdir, readFile, writeFile, stat } from 'fs/promises'
import { extname, join, dirname } from 'path'
import sharp from 'sharp'
import { ensureDirectoryExists } from './utils/files'

interface CLIArgs {
  outputPath?: string
  inputPath: string
  quality?: number
  keepOriginal?: boolean
}

type OptimizePngArgs = Omit<CLIArgs, 'quality' | 'keepOriginal'>

let quality: number
let keepOriginal: boolean

const optimizePng = async (args: OptimizePngArgs): Promise<void> => {
  try {
    const inputBuffer = await readFile(args.inputPath)

    const optimizedBuffer = await sharp(inputBuffer)
      .png({ quality })
      .toBuffer()

    if (args.outputPath) {
      await ensureDirectoryExists(dirname(args.outputPath))
      await writeFile(args.outputPath, optimizedBuffer)
      if (!keepOriginal) {
        await writeFile(args.inputPath, optimizedBuffer)
      }
    } else {
      await writeFile(args.inputPath, optimizedBuffer)
    }

    console.log(`Optimized PNG saved to ${args.outputPath || args.inputPath}`)
  } catch (error) {
    console.error('Error optimizing PNG:', error)
  }
}

const optimizeDirectory = async (args: CLIArgs): Promise<void> => {
  try {
    const files = await readdir(args.inputPath)

    for (const file of files) {
      const filePath = join(args.inputPath, file)
      const fileStat = await stat(filePath)
      const outputPath = args.outputPath ? join(args.outputPath, file) : undefined

      if (fileStat.isFile() && extname(file).toLowerCase() === '.png') {
        await optimizePng({ ...args, inputPath: filePath, outputPath })
      } else if (fileStat.isDirectory()) {
        await optimizeDirectory({ ...args, inputPath: filePath, outputPath })
      }
    }
  } catch (error) {
    console.error('Error optimizing directory:', error)
  }
}

export const optimize = async (args: CLIArgs): Promise<void> => {
  if (!args.inputPath) {
    console.error('Please provide an input path.')
    process.exit(1)
  }

  quality = +(args.quality || 80)
  if(quality < 1 || quality > 100) {
    console.error('Quality must be a number between 1 and 100.')
    process.exit(1)
  }

  keepOriginal = args.keepOriginal !== false

  try {
    const fileStat = await stat(args.inputPath)
    if (fileStat.isFile()) {
      await optimizePng(args)
    } else if (fileStat.isDirectory()) {
      await optimizeDirectory(args)
    } else {
      console.error('The provided path is neither a file nor a directory.')
    }
  } catch (error) {
    console.error('Error reading input path:', error)
  }
}
