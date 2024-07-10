import {AvifOptions, GifOptions, HeifOptions, Jp2Options, JpegOptions, JxlOptions, PngOptions, RawOptions, TiffOptions, WebpOptions} from 'sharp'

export type SuportedTypes = JpegOptions | PngOptions | JxlOptions | Jp2Options | WebpOptions | GifOptions | AvifOptions | HeifOptions | TiffOptions | RawOptions | { quality?: number; }
export type FileBuffer = Buffer | ArrayBuffer | Uint8Array | Uint8ClampedArray | Int8Array | Uint16Array | Int16Array | Uint32Array | Int32Array | Float32Array | Float64Array | string

export type CLIArgs = {
  outputPath?: string
  inputPath: string
  formatOptions?: SuportedTypes
  keepOriginal?: boolean
}

export type SupportedExtensions = 'jpeg' | 'png' | 'jxl' | 'jp2' | 'webp' | 'gif' | 'avif' | 'heif' | 'tiff' | 'raw'

export type OptimizePngArgs = Omit<CLIArgs, 'quality' | 'keepOriginal'>

