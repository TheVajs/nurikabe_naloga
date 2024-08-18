# Nurikabe Assigment OM

![program_screen_shot](https://github.com/TheVajs/nurikabe_naloga/tree/main/imgs/screen_shot.png?raw=true)
![screen_shot](https://github.com/user-attachments/assets/11ae8539-8a89-4401-8dfe-88674b96726b)
![Screenshot_20240817_014309](https://github.com/user-attachments/assets/cc8fed64-f17d-4664-9b59-1c528fa183e8)

<br />

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


# Run web page

## With python

1. Install newest version of python (has to be python3),

https://www.python.org/downloads/

2. Open bash/cmd in the project root directory (`/nurikabe_naloga`),
3. And now run command:
- `python -m http.server 8080` (but it seem to not work on Windows 11 for me).

Extra help: [How do you set up a local testing server?](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/Tools_and_setup/set_up_a_local_testing_server) (***includes command for python 2***)

- `python run_server.py`. This is a simple script that's included in the repo. Runs a simple local web server, similira to the first command. (tested on Windows 11 and  Linux, seem to work fine). 
4. Now the web site is available on <a href="http://localhost:8080">http://localhost:8080</a> or <a href="http://127.0.0.1:8080">http://127.0.0.1:8080</a>.

## With visual studio code

Install Extension

https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer

and then right click on `index.html` in the root directory and click "Open with Live Server"

