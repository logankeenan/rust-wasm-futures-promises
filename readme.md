# Mixing Rust Futures and JavaScript Promises with Web Assembly (WASM) 

This repo demos two things:

1. Passing a JS promise to rust and then converting it to a rust future and awaiting it.
2. Calling some async code in rust and awaiting for the result in javascript

## Running Locally

1. Compile to WASM:
    ```bash
    wasm-pack build --target web
    ```

1. Open index.html in the browser.  I used IntelliJ's built in http server.