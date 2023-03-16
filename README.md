# Rust Ceramic

Implementation of the Ceramic protocol in Rust.

Current status is that the `ceramic-one` binary only mimics the Kubo RPC API and relies on https://github.com/ceramicnetwork/js-ceramic for the remaining logic.

## Usage

Run in single binary using the `ceramic-one` crate:

    $ cargo run -p ceramic-one -- daemon

The process honors RUST_LOG env variable for controlling its logging output.
For example, to enable debug logging for code from this repo but error logging for all other code use:

    $ RUST_LOG=ERROR,ceramic_kubo_rpc=DEBUG,ceramic_one=DEBUG cargo run -p ceramic-one -- daemon

## Contributing

We are happy to accept small and large contributions, feel free to make a suggestion or submit a pull request.

Use the provided `Makefile` for basic actions to ensure your changes are ready for CI.

    $ make build
    $ make check-clippy
    $ make check-fmt
    $ make test

Using the makefile is not necessary during your developement cycle, feel free to use the relvant cargo commands directly.
However running `make` before publishing a PR will provide a good signal if you PR will pass CI.

## Releasing

Creating a new release of this project is a two step process.

1. Create a release PR
2. Merge and publish release

You can use `make release-pr` to create a release PR.
However we recommend you use the `Release PR` Github Action.
Once the release PR is merged CI will automatically publish the release using `make release`.

> NOTE: All crates in this repo use the same version number.

## License

Fully open source and dual-licensed under MIT and Apache 2.
