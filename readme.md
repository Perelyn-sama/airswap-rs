# airswap-rs

## Quickstart

Add this to your Cargo.toml:

```toml
[dependencies]
airswap-rs = { git = "https://github.com/Perelyn-sama/airswap-rs" }
```

And this to your code:

```rust
use airswap::prelude::*;
```

## Examples

Examples can be found [here][examples].

[examples]: https://github.com/Perelyn-sama/airswap-rs/tree/master/examples

## Roadmap

-   [ ] [Pool Contract](https://github.com/airswap/airswap-protocols/tree/develop/source/pool)
    -   [x] Implementation
    -   [x] Documentation
    -   [ ] Tests
-   [ ] [Staking Contract](https://github.com/airswap/airswap-protocols/tree/develop/source/staking)
    -   [x] Implementation
    -   [x] Documentation
    -   [ ] Tests
-   [ ] [Registry Contract](https://github.com/airswap/airswap-protocols/tree/develop/source/indexer-registry)
    -   [x] Implementation
    -   [x] Documentation
    -   [ ] Tests
-   [ ] [Wrapper Contract](https://github.com/airswap/airswap-protocols/tree/develop/source/wrapper)
    -   [x] Implementation
    -   [x] Documentation
    -   [ ] Tests
-   [ ] [Swap-ERC20 Contract](https://github.com/airswap/airswap-protocols/tree/develop/source/swap-erc20)
  -   [x] Implementation
  -   [x] Documentation
  -   [ ] Tests
-   [x] [Contract addresses](src/contracts/addresses.json)
    -   [x] Pool Contract Addresses
    -   [x] Staking Contract Addresses 
    -   [x] Registry Contract Addresses
    -   [x] Wrapper Contract Addresses 
    -   [x] Swap-ERC20 Contract Addresses

## License

This project is licensed under the [MIT license](https://github.com/Perelyn-sama/airswap-rs/blob/master/LICENSE).
