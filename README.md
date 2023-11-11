# One bit jam

Web build framework adapted from https://github.com/NiklasEi/bevy_game_template/tree/main

Please see /credits/ for the licensing

---

# How to build

* Run the native version with `cargo run`
* Build the web version with: `trunk build`
   * requires [trunk](https://trunkrs.dev/): `cargo install --locked trunk`
   * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
* All resultant files can be found in dist. Serve with e.g. npm [http-server](https://www.npmjs.com/package/http-server), on windows:
   * navigate to /dist/: `cd dist`
   * requires npm: `npx http-server .`
   * game will be served on port 8080

# Web Version

No remote hosting yet.
