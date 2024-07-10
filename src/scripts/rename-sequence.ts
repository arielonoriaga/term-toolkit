import { readdir, rename } from "fs/promises";
import { join, extname, isAbsolute } from "path";
import { homedir } from "os";

async function renameFiles(directory: string, baseName: string): Promise<void> {
  try {
    const files: string[] = await readdir(directory);
    const sortedFiles = files.sort();

    const totalFiles = files.length;
    const padLength = totalFiles.toString().length;

    for (let i = 0; i < totalFiles; i++) {
      const file = sortedFiles[i];
      const fileExtension = extname(file);
      const paddedNumber = i.toString().padStart(padLength, '0');
      const newFileName = `${baseName}${paddedNumber}${fileExtension}`;
      const oldFilePath = join(directory, file);
      const newFilePath = join(directory, newFileName);

      try {
        await rename(oldFilePath, newFilePath);
        console.log(`Renamed ${oldFilePath} to ${newFilePath}`);
      } catch (err) {
        console.error(`Error renaming file ${oldFilePath}:`, err);
      }
    }
  } catch (err) {
    console.error(`Error reading directory ${directory}:`, err);
  }
}

const args: string[] = process.argv.slice(2);
if (args.length < 2) {
  console.error("Please provide a directory path and a base name for the files as arguments.");
  process.exit(1);
}

let directoryPath: string = args[0];

if (directoryPath.startsWith('~')) {
  directoryPath = join(homedir(), directoryPath.slice(1));
}

if (!isAbsolute(directoryPath)) {
  directoryPath = join(process.cwd(), directoryPath);
}

const baseName: string = args[1];
renameFiles(directoryPath, baseName);
