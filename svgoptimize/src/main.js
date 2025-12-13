import { optimize } from 'svgo';
import fs from 'fs';

let dir = await fs.promises.readdir('../temp/');
for (let file of dir) {
    if (file.endsWith('.svg')) {
        let svg = await fs.promises.readFile(`../temp/${file}`, 'utf-8');
        let result = optimize(svg, {
            plugins: [
                {
                    name: 'convertPathData',
                    params: {
                        noSpaceAfterFlags: false,
                    },
                },
                "removeUselessDefs",
                {
                    name: "mergePaths",
                    params: {
                        force: true,
                        floatPrecision: 10,
                        noSpaceAfterFlags: false
                    }
                },
            ]
        })
        await fs.promises.writeFile(`../temp/${file}`, result.data, 'utf-8');
    }
}