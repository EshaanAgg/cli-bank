# Conway's Game of Life

Using Rust and WASM integration to create Conway's Game of Life simulation that can run on the web!

### Technical Nuances

All the Rust code is located in the [src](./src/) directory, and we use the `wasm-pack build` command to generate JavaScript bindings for the same in a `pkg` directory, which is not commited to the source control.

We also have used the `create-wasm-app` package from `npm` to create a starter project (that uses the Rust generated WebAssembly and bundles them with Webpack) for using the same in the [`www`](./www/) directory. In this project, we basically declare the `pkg` folder as a dependency in `npm`, and then use `webpack` to setup a live development server.

You might need to run the following in order to support the webpack server:

```bash
export NODE_OPTIONS=--openssl-legacy-provider
```

#### JavaScript - Rust Interface Design

As a general rule of thumb, a good `JavaScript` - `WebAssembly` interface design is often one where large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory, and then are exposed to JavaScript as opaque handles. JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result. By only returning the small result of the computation, we avoid copying and/or serializing everything back and forth between the JavaScript's garbage-collected heap and the WebAssembly linear memory.

Initially, we were encoding the Universe as a string that can be send to JavaScript engine to render, but generating (and allocating) a `String` in Rust and then having `wasm-bindgen` convert it to a valid JavaScript string makes unnecessary copies of the universe's cells. As the JavaScript code already knows the width and height of the universe, and can read WebAssembly's linear memory that make up the cells directly, we subsequently modify the render method to return a pointer to the start of the cells array. This then allows JavaScript to read the memory directly and create the cells on the canvas.

#### Running tests

You can run the tests with the help of the following command:

```bash
wasm-pack test --firefox --headless
```
