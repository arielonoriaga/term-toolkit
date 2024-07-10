import { readdir, readFile, writeFile, stat } from 'fs/promises'
import { extname, join, dirname } from 'path'
import sharp, {AvifOptions, GifOptions, HeifOptions, Jp2Options, JpegOptions, JxlOptions, PngOptions, RawOptions, TiffOptions, WebpOptions} from 'sharp'
import { ensureDirectoryExists } from './utils/files'

type SuportedTypes = JpegOptions | PngOptions | JxlOptions | Jp2Options | WebpOptions | GifOptions | AvifOptions | HeifOptions | TiffOptions | RawOptions | { quality?: number }
type FileBuffer = Buffer | ArrayBuffer | Uint8Array | Uint8ClampedArray | Int8Array | Uint16Array | Int16Array | Uint32Array | Int32Array | Float32Array | Float64Array | string

type CLIArgs = {
  outputPath?: string
  inputPath: string
  formatOptions?: SuportedTypes
  keepOriginal?: boolean
}

type OptimizePngArgs = Omit<CLIArgs, 'quality' | 'keepOriginal'>

let quality: number
let keepOriginal: boolean

type SupportedExtensions = 'jpeg' | 'png' | 'jxl' | 'jp2' | 'webp' | 'gif' | 'avif' | 'heif' | 'tiff' | 'raw'

const isSupportedExtension = (extension: string): extension is SupportedExtensions => {
  return ['jpeg', 'png', 'jxl', 'jp2', 'webp', 'gif', 'avif', 'heif', 'tiff', 'raw'].includes(extension)
}

const sharpMethodByExtension = (file: FileBuffer, fileExtension: SupportedExtensions) => ({
  jpeg: (options?: JpegOptions) => sharp(file).jpeg(options),
  png: (options?: PngOptions) => sharp(file).png(options),
  jxl: (options?: JxlOptions) => sharp(file).jxl(options),
  jp2: (options?: Jp2Options) => sharp(file).jp2(options),
  webp: (options?: WebpOptions) => sharp(file).webp(options),
  gif: (options?: GifOptions) => sharp(file).gif(options),
  avif: (options?: AvifOptions) => sharp(file).avif(options),
  heif: (options?: HeifOptions) => sharp(file).heif(options),
  tiff: (options?: TiffOptions) => sharp(file).tiff(options),
  raw: (options?: RawOptions) => sharp(file).raw(options),
}[fileExtension])

const optimizePng = async (args: OptimizePngArgs): Promise<void> => {
  try {
    const inputBuffer = await readFile(args.inputPath) as FileBuffer
    const fileExtension = extname(args.inputPath).toLowerCase().replace('.', '') as SupportedExtensions

    const sharpMethod = sharpMethodByExtension(inputBuffer, fileExtension)
    if (!sharpMethod) {
      console.error('Unsupported file format.')
      return
    }

    const optimizedBuffer = await sharpMethod({
      ...(args.formatOptions || {}),
      quality,
    }).toBuffer()

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

      if (fileStat.isFile() && isSupportedExtension(extname(file).toLowerCase().replace('.', ''))) {
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

  // @ts-ignore
  quality = +(args.formatOptions?.quality || 80)
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
