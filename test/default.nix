{ pkgs }:
let
 sandbox-test = pkgs.writeShellScriptBin "sandbox-test"
 ''
 set -euxo pipefail
 mkdir -p dist
 hn-rust-fmt-check
 hn-rust-clippy
 cargo test
 hc package -o dist/sandbox.dna.json
 hc test --skip-package
 '';
in
{
 buildInputs = [ sandbox-test ];
}
