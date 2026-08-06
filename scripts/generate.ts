import { $ } from "bun";
import { runStep2 } from "./step2";

export const LUCIDE_TEMPDIR = "./temp/lucide";
export const LUCIDE_LAB_TEMPDIR = "./temp/lab";

export const LUCIDE_SOURCEDIR = "./node_modules/lucide-static/icons";
export const LUCIDE_LAB_SOURCEDIR = "./node_modules/@lucide/lab/icons";

export default async function generate() {
    await $`cargo run -- step1`;
    await runStep2(LUCIDE_TEMPDIR);
    await runStep2(LUCIDE_LAB_TEMPDIR);
    await $`cargo run -- step3`;
}
