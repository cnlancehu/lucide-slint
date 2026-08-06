import { readFile } from "node:fs/promises";
import { writeFile } from "node:fs/promises";
import { readdir } from "node:fs/promises";
import path from "node:path";
import { optimize } from "svgo";

export async function runStep2(source: string) {
    const files = await readdir(source);
    for (let file of files) {
        let contentPath = path.join(source, file);
        let content = await readFile(contentPath, "utf-8");
        let result = optimize(content, {
            plugins: [
                {
                    name: "convertPathData",
                    params: {
                        noSpaceAfterFlags: false,
                    },
                },
                "removeUselessDefs",
                {
                    name: "mergePaths",
                    params: {
                        floatPrecision: 5,
                        noSpaceAfterFlags: false,
                    },
                },
            ],
        });
        await writeFile(contentPath, (result as any).data, "utf-8");
    }
}
