import { homedir } from "os";
import { readdir, unlink } from "fs/promises";
import { join, isAbsolute } from "path";
import {isEven} from "./utils/numbers";

const _deleter = async (directory: string, even: boolean): Promise<void> => {
  const shouldBeDeleted = (index: number): boolean =>  even
    ? isEven(index)
    : !isEven(index);

  try {
    const files: string[] = await readdir(directory);

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      const filePath = join(directory, file);

      if (shouldBeDeleted(i)) {
        try {
          await unlink(filePath);
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

export const deleter = async (directory: string, even: boolean): Promise<void> => {
  if (!directory) {
    console.error("Please provide a directory path.");
    process.exit(1);
  }

  if (directory.startsWith('~')) {
    directory = join(homedir(), directory.slice(1));
  }

  if (!isAbsolute(directory)) {
    directory = join(process.cwd(), directory);
  }

  _deleter(directory, even);
}
