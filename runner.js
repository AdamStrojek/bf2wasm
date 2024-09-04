const fs = require('node:fs');

const memory = new WebAssembly.Memory({ initial: 1 });

const importObject = {
    env: {
        memory: memory,
        putch(arg) {
            process.stdout.write(String.fromCharCode(arg));
        }
    }
};

const wasmBuffer = fs.readFileSync(process.argv[2]);
WebAssembly.instantiate(wasmBuffer, importObject).then(wasmModule => {
  // Exported function live under instance.exports
  const { bf_wasm } = wasmModule.instance.exports;

  console.log('Running BF program...');
  const ptr = bf_wasm();
  console.log('Finished');
  console.log(`Final ptr position: ${ptr}`);
  const mem = new Uint8Array(memory.buffer);
  console.log(`Memory view: ${mem.slice(0, 16)}`);
});
