# asrpgenigma

# Auto-Swipe RPG Script
This project provides a WebAssembly (WASM) script to automate swiping/clicking monsters and upgrades in a web-based RPG game. The script uses wasm-bindgen to interact with the browser's DOM, periodically clicking elements like monsters (e.g., .monster) and upgrades (e.g., .upgrade).

# Prerequisites

Rust: Install from rustup.rs.
wasm-pack: Install with cargo install wasm-pack.
Web Server: Use http-server (npm install -g http-server) or Python (python3 -m http.server) to serve files.
Browser: Modern browser supporting WebAssembly and ES modules (e.g., Chrome, Firefox).

Project Structure
After building, the project includes:

src/lib.rs: Rust source code for the auto-swipe logic.
Cargo.toml: Rust project configuration.
index.html: Example HTML file for (for testing purposes).
pkg/ (generated automatically after running wasm-pack build):
atsrpg.js: JavaScript glue code to load the WASM module.
atsrpg.wasm: Compiled WebAssembly binary with the script logic.



# Setup

Clone or Set Up the Project:

If starting fresh:cargo new --lib atsrpg
cd atsrpg


Ensure Cargo.toml and src/lib.rs match the provided files.


Build the WASM Module:
wasm-pack build --target web


This generates the pkg/ directory with atsrpg.js and atsrpg.wasm that are essential for web compiler interpretation :)


Serve the Files:

Place index.html and pkg/ in the project root.
Start a local web server:http-server -c-1
(Ensure httpserver is installed using the npm tool)

Access at http://localhost:8080.


# Test the Script:

Open http://localhost:8080 in a browser.
Check the console (F12, "Console" tab) for logs like: 
- Clicked monster #0
- Clicked upgrade #0

# Tihihi :)
