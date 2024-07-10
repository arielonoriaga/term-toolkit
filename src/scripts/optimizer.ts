import { readdir, readFile, writeFile, stat } from "fs/promises";
import { extname, join } from "path";
import sharp from "sharp";

type CLIArgs = {
  outputPath?: string;
  inputPath: string;
  quality?: number;
}

type OptimizePngArgs = Omit<CLIArgs, 'quality'>

let quality: number;

const optimizePng = async (args: OptimizePngArgs): Promise<void> => {
  try {
    const inputBuffer = await readFile(args.inputPath);

    const optimizedBuffer = await sharp(inputBuffer)
      .png({ quality })
      .toBuffer();

    await writeFile(args.outputPath || args.inputPath, optimizedBuffer);

    console.log(`Optimized PNG saved to ${args.outputPath || args.inputPath}`);
  } catch (error) {
    console.error("Error optimizing PNG:", error);
  }
};

const optimizeDirectory = async (args: CLIArgs): Promise<void> => {
  try {
    const files = await readdir(args.inputPath);

    for (const file of files) {
      const filePath = join(args.inputPath, file);
      const fileStat = await stat(filePath);

      if (fileStat.isFile() && extname(file).toLowerCase() === ".png") {
        await optimizePng({inputPath: filePath});
      } else if (fileStat.isDirectory()) {
        await optimizeDirectory({inputPath: filePath});
      }
    }
  } catch (error) {
    console.error("Error optimizing directory:", error);
  }
};

export const optimize = async (args: CLIArgs): Promise<void> => {
  if (!args.inputPath) {
    console.error("Please provide an input path.");
    process.exit(1);
  }

  quality = args.quality || 80;

  stat(args.inputPath)
    .then(fileStat => {
      if (fileStat.isFile()) {
        optimizePng(args);
      } else if (fileStat.isDirectory()) {
        optimizeDirectory(args);
      } else {
        console.error("The provided path is neither a file nor a directory.");
      }
    })
    .catch(error => {
      console.error("Error reading input path:", error);
    });
}
