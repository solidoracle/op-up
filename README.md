# op-up

> **Warning**
>
> This is a work in progress.

**OP-Up is a hive tool for testing OP-Stack-compatible software modules.**

This project was born out of the need to test out [Magi](https://github.com/a16z/magi), a rollup client built for the OP stack. Having an easy-to-use environment to spin up a local devnet is crucial for quick testing and experimentation, especially when there exist different implementations of each component in the stack.

For instance, you can use OP-Up to spin up a devnet with a [Geth](https://github.com/ethereum/go-ethereum) L1 node, an [OP-Erigon](https://github.com/testinprod-io/op-erigon) L2 node, and a [Magi](https://github.com/a16z/magi) rollup node, and test out the interoperability between them in an end-to-end fashion.

OP-Up makes use of the Git `sparse-checkout` functionality to clone only the necessary submodules of each component from the [Optimism](https://github.com/ethereum-optimism/optimism) and [Optimism-rs](https://github.com/refcell/optimism-rs) monorepos, and also features a custom Docker configuration.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Make](https://www.gnu.org/software/make/)
- [jq](https://jqlang.github.io/jq/)

## Usage

Clone this repository and run the following command from the root directory:

```sh
make devnet
```

## Bugs & Contributions

Please report any bugs or issues you encounter by opening an issue on GitHub. <br />
Contributions are welcome!

## License

This project is licensed under the [MIT License](LICENSE). <br />
Free and open-source, forever.
