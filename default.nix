let tools = import ./tools.nix; in with tools;

stdenv.mkDerivation {
    name = "web-compress";
    unpackPhase = ''
    cp -r ${./Cargo.toml} Cargo.toml
    cp -r ${./style.css} style.css
    cp -r ${./.cargo} .cargo
    cp -r ${./index.html} index.html
    cp -r ${./index.js} index.js
    cp -r ${./src} src
    '';
    buildInputs = [wasm-bindgen-cli cargo];
    buildPhase = ''
    mkdir site

    cp index.html style.css index.js site

    cargo build --release --target wasm32-unknown-unknown

    ls ./target/wasm32-unknown-unknown/release/

    wasm-bindgen \
        --no-typescript \
        --target web \
        --out-dir site \
        ./target/wasm32-unknown-unknown/release/*.wasm
    '';
    installPhase = ''
    cp -r site $out
    '';
}
