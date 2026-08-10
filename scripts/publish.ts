import { $, semver, TOML } from "bun";

await $`bun update`;

import lucideMeta from "lucide-static/package.json";
import lucideLabMeta from "@lucide/lab/package.json";
import { exit } from "process";
import generate from "./generate";
import { appendFile, writeFile } from "fs/promises";
import { buildDist } from "./dist";

let lucideVer = lucideMeta.version;
let lucideLabVer = lucideLabMeta.version;

let cargoTomlText = await Bun.file("./lucide-slint/Cargo.toml").text();
let cargoToml = TOML.parse(cargoTomlText) as any;

let semverOrder = semver.order(lucideVer, cargoToml.package.version);

if (semverOrder === 1) {
    await generate();
    cargoToml.package.version = lucideVer;
    // @ts-ignore
    writeFile("./lucide-slint/Cargo.toml", TOML.stringify(cargoToml));
    writeFile(
        "release-notes.md",
        `### Lucide ${lucideVer} with Lucide Lab ${lucideLabVer}\nVersion bump`,
    );
    const githubEnv = process.env.GITHUB_ENV;
    if (githubEnv) {
        await appendFile(githubEnv, `LUCIDE_VERSION=${lucideVer}\n`, "utf8");
    }

    await buildDist(lucideVer);

    console.log(
        "Finished, use `cargo publish -p lucide-slint --allow-dirty` to publish the new version.",
    );
} else {
    console.log("Already up-to-date");
    exit(11);
}
