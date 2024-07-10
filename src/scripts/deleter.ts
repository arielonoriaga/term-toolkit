import { homedir } from "os";
import { readdir, unlink } from "fs/promises";
import { join, isAbsolute } from "path";

 export const listAndDeleteFiles = async (directory: string): Promise<void> => {
  try {
    const files: string[] = await readdir(directory);
    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      const filePath = join(directory, file);
      if (i % 2 !== 0) { // If the index is not even
        try {
          await unlink(filePath); // Delete the file
          console.log(`Deleted file: ${filePath}`);
        } catch (err) {
          console.error(`Error deleting file ${filePath}:`, err);
        }
      } else {
        console.log(`Kept file: ${filePath}`);
      }
    }
  } catch (err) {
    console.error(`Error reading directory ${directory}:`, err);
  }
}

const args: string[] = process.argv.slice(2);
if (args.length === 0) {
  console.error("Please provide a directory path as an argument.");
  process.exit(1);
}

let directoryPath: string = args[0];

if (directoryPath.startsWith('~')) {
  directoryPath = join(homedir(), directoryPath.slice(1));
}

if (!isAbsolute(directoryPath)) {
  directoryPath = join(process.cwd(), directoryPath);
}
listAndDeleteFiles(directoryPath);
