import { copy_file, initSync } from "../pkg/r.js";

const wasm = Deno.readFileSync("./pkg/r_bg.wasm");
initSync(wasm.buffer);

globalThis.readFile = (path: string) => {
  return Deno.readTextFile(path);
};

globalThis.writeFile = (path: string, content: string) => {
  return Deno.writeTextFile(path, content);
};

copy_file("assets/test.txt", "assets/test2.txt");
