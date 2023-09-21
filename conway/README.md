# Conway's Game of Life

We will be using Rust and WASM integration to create Conway's Game of Life simulation that can run in the web.

### Technical Nuances

We write all our Rust code in the [src](./src/) directory and then use `wasm-pack build` command to generate JavaScript bindings for the same in a `pkg` directory, which is not commited to the source control.

We have used the `create-wasm-app` package from NPM to create a starter project (that uses the Rust generated WebAssembly and bundles them with Webpack) for using the same in the [`www`](./www/) directory. In this project, we basically declare the `pkg` folder as a dependency in NPM, and then use `webpack` to setup a live development server.

You might need to run the following in order to support the webpack server:

```bash
export NODE_OPTIONS=--openssl-legacy-provider
```

#### JavaScript - Rust Interface Design

As a general rule of thumb, a good `JavaScript` - `WebAssembly` interface design is often one where large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory, and are exposed to JavaScript as opaque handles. JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result. By only returning the small result of the computation, we avoid copying and/or serializing everything back and forth between the JavaScript's garbage-collected heap and the WebAssembly linear memory.
