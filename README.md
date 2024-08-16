![Nurikabe web program](https://github.com/TheVajs/nurikabe_naloga/tree/main/imgs/screen_shot.png?raw=true)

# Nurikabe Assigment OM

## How to install? (only for compiling)

1. install rust

https://www.rust-lang.org/tools/install

2. install wasm-pack with

cargo install wasm-pack
(requires C++ tools, 2017 or greater)

3. run with

```bash
wasm-pack build --target web
```

or with cargo-watch

```bash
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target web"
```

##### Resources:

Run wasm: https://sebhastian.com/how-to-print-javascript/<br />
Video: https://www.youtube.com/watch?v=nW71Mlbmxt8&ab_channel=AustinCrim<br />
link: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm<br />


## Run the program/web page

### With visual studio code

Install Extension

https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer

and then right click on `index.html` in the root directory and click "Open with Live Server"

### With npm


