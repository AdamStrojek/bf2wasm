const fs = require('node:fs');

const memory = new WebAssembly.Memory({ initial: 1 });

const fn_name = process.argv[3] || "main";
const input = process.argv[4] || "";
let input_pos = 0;

const importObject = {
    env: {
        memory: memory,
        putch(arg) {
            process.stdout.write(String.fromCharCode(arg));
        },
        getch() {
            return input[input_pos++];
        }
    }
};

const wasmBuffer = fs.readFileSync(process.argv[2]);
WebAssembly.instantiate(wasmBuffer, importObject).then(wasmModule => {
  // Exported function live under instance.exports
    const fn = wasmModule.instance.exports[fn_name];

    console.log('BF module begin');
    console.log(`Input: ${input}`);
    const ptr = fn();
    console.log('BF module finished');
    console.log(`Pointer: ${ptr}`);
    const mem_viewer = new Uint8Array(memory.buffer);
    console.log(`Memory slice: ${mem_viewer.slice(0, 16)}`);
});
