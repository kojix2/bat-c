## TypeScript (Node.js) examples

These examples use Node-API (N-API) to create native addons that call the bat-c library.

### Build and run

```sh
# From repo root
cargo build --release

cd examples/TypeScript
npm install  # This will run node-gyp rebuild
node basic.js
node self_print.js
```

### Manual build

```sh
cargo build --release
cd examples/TypeScript
node-gyp configure
node-gyp build
node basic.js
node self_print.js
```

Notes: Uses Node-API (N-API) to create native C++ addons that link to `libbat_c`. The `binding.gyp` file configures include paths, library linking, and rpath. The JavaScript files load the compiled `.node` addons and call the exported functions.
