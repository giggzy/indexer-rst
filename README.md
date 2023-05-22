# Explore building and querying a search index

## TODO

- [ ] process data into rust structs
- [ ] save to db
- [ ] tokenize and build index
- [ ] query index
- [ ] lookup addition information not stored in index

## Process Wikipedia XML

`cargo run [xml filename]`

## Misc

### Cargo watch

Watches for code changes and rebuilds binary.

`cargo watch [command]`

Available on nix as _cargo-watch_

### cargo-nextest

This is a next gen test runner with more work done on reports, more features and a speedup

`cargo nextest run  # run all tests`

Installed using nix, its called _cargo-nextest_ in [nixpkgs](https://search.nixos.org/packages?channel=22.11&from=0&size=50&sort=relevance&type=packages&query=cargo+nextest).

Not tested running on Github Actions yet but maybe worth using the [pre-build binary](https://nexte.st/book/pre-built-binaries.html#using-nextest-in-github-actions)
