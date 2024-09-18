(async () => {
    const memory = new WebAssembly.Memory({ initial: 1 });

    const importObject = {
        env: {
            memory: memory,
            putch(arg) {
                document.getElementById("output").innerHTML += String.fromCharCode(arg);
            }
        }
    };

    document.getElementById("start").onclick = (ev) => {
        const url = document.getElementById("module_path").value;

        const module = await WebAssembly.instantiateStreaming(fetch(url), importObject);

        // Exported function live under instance.exports
        const { bf_wasm } = wasmModule.instance.exports;

        const ptr = bf_wasm();

        const mem_viewer = new Uint8Array(memory.buffer);
        document.getElementById("memory").innerHTML = `Memory slice: ${mem_viewer.slice(0, 16)}`;
    };

})();
