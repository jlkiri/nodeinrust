import fs from "fs";
import path from "path";
import { parse } from "gyp-parser";

const NODE_PATH = "node";

const gyp_exists = fs.existsSync(path.resolve(`${NODE_PATH}/node.gyp`));

if (!gyp_exists) {
  throw Error(
    "node.gyp not found! Make sure to fetch files from node repository first!"
  );
}

// Inject C++ embedder API into Node's source

console.log("Copying C++ embedder API files to Node's source...");

const copy_src = ["nodec.cc", "nodec.h"];

for (let src of copy_src) {
  fs.copyFileSync(
    path.resolve("nodec", src),
    path.resolve(`${NODE_PATH}/src/${src}`)
  );
}

// Add copied files to gyp build configuration

console.log("Reading and modifying node.gyp...");

const raw_gyp = fs.readFileSync(`${NODE_PATH}/node.gyp`, { encoding: "utf8" });
const gyp = parse(raw_gyp);

const targets = gyp.targets.find(
  (t) => t.target_name === "<(node_lib_target_name)"
)
const cpp_source_list = targets.sources;

// const win = targets.conditions.find(cond => cond[0] == 'OS=="win"');

// win[1].libraries.push("Winmm")
cpp_source_list.push("src/nodec.cc", "src/nodec.h");

const serialized = JSON.stringify(gyp, null, 2);
fs.writeFileSync(`${NODE_PATH}/node.gyp`, serialized, { encoding: "utf8" });

console.log("SUCCESS!");
