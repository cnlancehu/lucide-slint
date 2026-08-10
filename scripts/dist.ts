import { mkdir } from "node:fs/promises";
import { rm } from "node:fs/promises";

export async function buildDist(version: string) {
    try {
        await rm("./dist");
        await mkdir("./dist");
    } catch {}

    let lucide_slint = await Bun.file("./lucide-slint/lucide.slint").text();
    let lucide_lab_slint = await Bun.file("./lucide-slint/lucide-lab.slint").text();

    await Bun.write("./dist/lucide.slint", lucide_slint);
    await Bun.write("./dist/lucide-lab.slint", lucide_lab_slint);

    let cmakePackageFiles: Record<string, string> = {};
    cmakePackageFiles[`lucide-slint-${version}/lucide.slint`] = lucide_slint;
    cmakePackageFiles[`lucide-slint-${version}/lucide-lab.slint`] = lucide_lab_slint;
    cmakePackageFiles[`lucide-slint-${version}/CMakeLists.txt`] = cmakeTemplate(version);

    const archive = new Bun.Archive(cmakePackageFiles, { compress: "gzip" });
    await Bun.write("./dist/cmake.tar.gz", archive);
}

function cmakeTemplate(version: string) {
    return `cmake_minimum_required(VERSION 3.21)

project(
    LucideSlint
    VERSION ${version}
    LANGUAGES NONE
)

set(
    LucideSlint_LUCIDE_LIBRARY_PATH
    "lucide=\${CMAKE_CURRENT_LIST_DIR}/lucide.slint"
)

set(
    LucideSlint_LUCIDE_LAB_LIBRARY_PATH
    "lucide-lab=\${CMAKE_CURRENT_LIST_DIR}/lucide-lab.slint"
)

set(
    LucideSlint_LIBRARY_PATHS
    "\${LucideSlint_LUCIDE_LIBRARY_PATH}"
    "\${LucideSlint_LUCIDE_LAB_LIBRARY_PATH}"
)

set(
    LucideSlint_LUCIDE_LIBRARY_PATH
    "\${LucideSlint_LUCIDE_LIBRARY_PATH}"
    PARENT_SCOPE
)

set(
    LucideSlint_LUCIDE_LAB_LIBRARY_PATH
    "\${LucideSlint_LUCIDE_LAB_LIBRARY_PATH}"
    PARENT_SCOPE
)

set(
    LucideSlint_LIBRARY_PATHS
    "\${LucideSlint_LIBRARY_PATHS}"
    PARENT_SCOPE
)`;
}
