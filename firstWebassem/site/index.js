const js = import("../hello-wasm/pkg/hello_wasm");
js.then(js => {
  js.greet("WebAssembly");
});