import { mkdir } from 'node:fs/promises'

export const ensureDirectoryExists = async (directoryPath: string): Promise<void> => {
  try {
    await mkdir(directoryPath, { recursive: true })
  } catch (error) {
    console.error(`Error creating directory ${directoryPath}:`, error)
  }
}
