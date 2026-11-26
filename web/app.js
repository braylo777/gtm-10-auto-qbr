// Loads the Rust core (app.wasm) and runs the selected example. No dependencies.
const enc = new TextEncoder(), dec = new TextDecoder();
let wasm;
async function load() {
  const bytes = await (await fetch("app.wasm")).arrayBuffer();
  wasm = (await WebAssembly.instantiate(bytes, {})).instance.exports;
}
function call(req) {
  const inp = enc.encode(req);
  const p = wasm.alloc(inp.length);
  new Uint8Array(wasm.memory.buffer, p, inp.length).set(inp);
  const packed = wasm.run(p, inp.length);
  wasm.dealloc(p, inp.length);
  const op = Number(packed >> 32n), ol = Number(packed & 0xffffffffn);
  const out = dec.decode(new Uint8Array(wasm.memory.buffer, op, ol).slice());
  wasm.dealloc(op, ol);
  return out;
}
const cfg = JSON.parse(document.getElementById("cfg").textContent);
const tabs = document.getElementById("tabs"), help = document.getElementById("help");
const input = document.getElementById("input"), output = document.getElementById("output");
let cur = 0;
function pick(i) {
  cur = i;
  [...tabs.children].forEach((b, j) => b.setAttribute("aria-selected", String(i === j)));
  help.textContent = cfg.ops[i].help;
  input.value = JSON.stringify(cfg.ops[i].input, null, 2);
  output.textContent = "Press Run.";
  output.className = "";
}
cfg.ops.forEach((o, i) => {
  const b = document.createElement("button");
  b.textContent = o.label; b.type = "button"; b.setAttribute("role", "tab");
  b.onclick = () => pick(i); tabs.appendChild(b);
});
document.getElementById("run").onclick = () => {
  let parsed;
  try { parsed = JSON.parse(input.value); } catch (e) { output.textContent = "Input is not valid JSON: " + e.message; output.className = "err"; return; }
  const res = JSON.parse(call(JSON.stringify({ op: cfg.ops[cur].op, input: parsed })));
  output.textContent = JSON.stringify(res.ok ? res.result : res, null, 2);
  output.className = res.ok ? "ok" : "err";
};
document.getElementById("reset").onclick = () => pick(cur);
pick(0);
load().then(() => document.getElementById("run").disabled = false)
  .catch(e => { output.textContent = "Could not load app.wasm: " + e; output.className = "err"; });
