# Changelog

All notable changes to this project will be documented in this file.

## [3.5.2] - 2024-11-02

### Features

- Add versioned-dirs flag [e81774e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e81774e00b8cfce79ed45d61abb0e1c0233a9832)

## [3.5.0] - 2024-11-01

### Miscellaneous Tasks

- Release v3.5.0 [ff7d646](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ff7d646f6f800979d28b8e1b4b7c193eddd3c40c)

### Refactor

- Migrate some ideas and move the compression logic to roast internally [901e2f7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/901e2f78280ab2c0bb2651a494f5db92485438f2)

## [3.3.3] - 2024-10-18

### Bug Fixes

- Properly set which version to set in the manifest [f0051fe](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f0051feb263276815845a35089f7fdb390fd44d0)

### Miscellaneous Tasks

- Release v3.3.3 [a1669f3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a1669f3593b05e222fea54e23793a6ac22dca555)

## [3.3.2] - 2024-10-18

### Miscellaneous Tasks

- Release v3.3.2 [09f0874](https://github.com/openSUSE-Rust/obs-service-cargo/commit/09f08749479be999dae8f36ce60d1037b9005f27)

## [3.3.1] - 2024-10-18

### Miscellaneous Tasks

- Release v3.3.1 [c9a7d47](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c9a7d4757f1ff14feecca2a478bf555461150480)

### Cargo

- Update lockfile and bump crate versions [0dbb45d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0dbb45d0e9b9a4d9b2d4131bec10bdb8a45936d7)

### Cli

- Improve description [e736094](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e736094899058fb6f04576411713039d35f91578)

## [3.0.0] - 2024-10-15

### Features

- Support uncompressed tarballs or with `.tar` extension [5733edc](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5733edcf50b494441d8d854a490e0f154934b4f5)

### Ci

- Format check should not fail ci [fa14c74](https://github.com/openSUSE-Rust/obs-service-cargo/commit/fa14c74101c6ed3b1cc047b9b57de87eb3268d47)

### Crates

- Get libroast dep from crates.io [043c050](https://github.com/openSUSE-Rust/obs-service-cargo/commit/043c05053c5db9e5cb697c4aabd80d0f319ec7a7)

## [2.0.0] - 2024-10-12

### Miscellaneous Tasks

- V2.0.0 [d775b7c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d775b7c006f537f704187928d7a53d8418f11e7f)

### Refactor

- Successful move to use libroast [8c2cda4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8c2cda44209556177285ad4de567aa0f7d0bc0d9)

## [1.4.1] - 2024-10-09

### Miscellaneous Tasks

- V1.4.1 [7a2b1cf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7a2b1cf1c81656c51d4ba00fccc72dcdfbfc37c2)

### Clippy

- Use inspect when map returns the original item (#98) [7a62324](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7a623244fa741f1061840f433bc7211e0b3f6674)

### Minor

- Add `--versioned-dirs` flag [1e728a2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1e728a2804ed064e7280697523124381e9c5e0af)

## [1.3.6] - 2024-06-06

### Bug Fixes

- Vendor-filterer --all-features rather than --all-features=true (#87) [89b17bf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/89b17bf80fbb416c415a247ea675818506b14b94)

### Documentation

- Fix warning indicator [8ef870e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8ef870e9ec4572925af6c694843b133612babb97)
- Fix again the warn indicator [837ac22](https://github.com/openSUSE-Rust/obs-service-cargo/commit/837ac221a05c63b8b6fca275c27d66e3554fd708)
- Fix warning indicator (#89) [6effe77](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6effe77bfdcc4036c4915dc83586b2aed589829d)

### Internet_points

- Add Pi-Cla (#88) [be74488](https://github.com/openSUSE-Rust/obs-service-cargo/commit/be7448838b011aa07fef649de866fdf533928cb3)

## [1.3.2] - 2024-05-02

### Miscellaneous Tasks

- Update to v1.3.2 [57973e0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/57973e030aef5a841bb8a89dca92c4fb5ca13df1)

## [1.3.1] - 2024-03-30

### Miscellaneous Tasks

- 1.3.1 [0aaa39b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0aaa39b4523163a1eeddbe7202063ec28e302046)

## [1.3.0] - 2024-03-30

### Documentation

- Add CONTRIBUTORS.md [0f71a3a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0f71a3afe832f710d76307ac154e8bc87d06157b)
- Update CHANGELOG [83e98c2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/83e98c2736fc0b15356f454c3ea4f59133ac0cec)

### Miscellaneous Tasks

- Add changelog with git-cliff [ee6cd1f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ee6cd1f926a7041478661c893b3fd9ffba3cef4e)

### Appreciation

- All who contributed code to this project should be listed [1b58973](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1b5897348a3f43f3ddde065899fde9b72af26346)

## [1.2.1] - 2024-03-07

### Miscellaneous Tasks

- V1.2.1 [4a877e3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4a877e3c4ec71fdcf755a4646d04e92060b60eee)

## [1.1.0] - 2024-03-02

### Bug Fixes

- Reconcile merge conflict [06cd879](https://github.com/openSUSE-Rust/obs-service-cargo/commit/06cd879b6fb99e5ec3da1de5a977ef8c823d57cc)

### Miscellaneous Tasks

- V1.1.0 [590f604](https://github.com/openSUSE-Rust/obs-service-cargo/commit/590f604a56c8985918a35e77ed8c4b1b9e5d3b00)

## [1.0.2] - 2024-03-02

### Bug Fixes

- Update versioning of dependencies to fix builds [cb26e0a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cb26e0a0338e729ec594f6f54d6f6da5802a3047)

### Miscellaneous Tasks

- V1.0.2 [389a9b9](https://github.com/openSUSE-Rust/obs-service-cargo/commit/389a9b9983a7688909df12365374045d3c37a73a)

## [1.0.1] - 2024-02-29

### Miscellaneous Tasks

- Update lockfile [cfba3eb](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cfba3eb6841013b88f22538914951536c37f68e5)

## [1.0.0] - 2024-01-14

### Miscellaneous Tasks

- V1.0.0 [16e9856](https://github.com/openSUSE-Rust/obs-service-cargo/commit/16e9856dafb7ccfbc823abfd7149b0fa49b3d4d2)

## [0.9.0] - 2023-12-24

### Miscellaneous Tasks

- V0.9.0 [104b2d4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/104b2d4aa31c7cadb8ed69ed970e5753b138062e)

### Clippy

- Apply clippy suggestions [643c65d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/643c65d905f5f44fb60751fa6d3803a538656485)

## [0.8.21] - 2023-12-11

### Miscellaneous Tasks

- V0.8.21 [cd9799b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cd9799b52aa43328a616813423222cd3563e0f9e)

## [0.8.20] - 2023-12-07

### Miscellaneous Tasks

- 0.8.20 [8cd5f30](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8cd5f30608bf8752f9e0ccca6e7b5413e47829e3)

## [0.8.19] - 2023-12-07

### Bug Fixes

- We added bz2 mime type which was forgotten to be added in consts.rs [39f3b45](https://github.com/openSUSE-Rust/obs-service-cargo/commit/39f3b45facd258622e509ecb0424b72742809d29)

### Miscellaneous Tasks

- 0.8.19 [d399d7a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d399d7a0c9563d537e0205aed62bd1d13316b8c0)

## [0.8.18] - 2023-12-05

### Miscellaneous Tasks

- 0.8.18 [ca11f4b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ca11f4bd58563e3ed9f4ad45cd68be8649823222)

## [0.8.15] - 2023-11-24

### Bug Fixes

- Check also the targets section [68d7eac](https://github.com/openSUSE-Rust/obs-service-cargo/commit/68d7eacbaa4ab1eae71eaa1e84931977a5c4b977)
- We also include build_dependencies [46560c6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/46560c674be831fbd024827b315d0293acdce257)

### Documentation

- Put warning on README [ci skip] [7e77a26](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7e77a268caf119a20445335967b221489bf34090)

### Miscellaneous Tasks

- Nitpick. remove small space [ci skip] [378bbfc](https://github.com/openSUSE-Rust/obs-service-cargo/commit/378bbfc9c7f6026a62c062ea299669d2a4b1fdb3)
- Release 0.8.15 [39f8c58](https://github.com/openSUSE-Rust/obs-service-cargo/commit/39f8c58ed8387f87dc5275f40c5edb07033bb942)

### Testing

- Add VBox-Starter in runtests.sh [cb8a9a3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cb8a9a3c432c87ca56b8abb9847b05f7f661669f)

## [0.8.12] - 2023-11-21

### Documentation

- Add possible vendor failures in README [ci skip] [7904a15](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7904a155dfe15cae36b0836db2be8ebd58df0ed8)

### Miscellaneous Tasks

- Bump to version 0.8.12 [f8ec1b4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f8ec1b4d107871a65673842398df8efda0610ca4)

### Improv

- Reintroduced tag in README [c67d7d2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c67d7d2f00bb0c07431d4707873c2c954fdedeb0)

## [0.8.10] - 2023-11-18

### Bug Fixes

- Check if dependencies section is empty or not [621dea1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/621dea1f84cd37f7a178bf5878c0e22354961c6b)

### Documentation

- Add status badge in the readme [06b24c0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/06b24c0c733c7d08bf7e838889a0b393f1ec85f8)
- Update README [ci skip] [4c15d8e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4c15d8e6f7dac07267d558199e71bb6939aeb75b)

### Miscellaneous Tasks

- Remove redundant messages [d0cbcd4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d0cbcd437c9f604566ac3fba6923ea6b8d14b8a4)

### Ci

- Install libzstd. it's needed [f7039e3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f7039e3c078c5c88f843712e8f4b4292c12df360)
- Put the tests into a convenient script [84cca41](https://github.com/openSUSE-Rust/obs-service-cargo/commit/84cca411fbf4b632bbc6b6eb16b19876154882f2)

### Improvement

- Better handling on when and when not to vendor [751775c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/751775c2c070481c9a755fa86faeb12c503d8729)

## [0.8.9] - 2023-11-17

### Bug Fixes

- Sometimes lockfiles can't be found [8b9ef3f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8b9ef3fec060927deaa57c67191a01e036d8b825)

### Miscellaneous Tasks

- Ready for patch release 0.8.9 [bf1ce2c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bf1ce2c3ce16a75b040685d7c913358b432266b4)

## [0.8.8] - 2023-11-17

### Bug Fixes

- Ensure `basepath` and sources are joined as one path [6f39308](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6f39308f0c7e814f6ab6e345f79f25199747eb73)
- Also join the paths from `prjdir` [ec3b628](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ec3b628c2de748e2f5600a0d36d7284d3e968f48)
- Hasdeps should inherit should_vendor value [696a35a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/696a35a909605f0b059d2974aaed30b11d500b37)
- Just warn and don't return an error [b95565f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b95565feed61bf4107fd45baee33d44d50879dbb)

### Documentation

- Add dependencies section in the README [9609059](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9609059b9357e9b0d449539da5d8abaf00d8f1cc)

### Miscellaneous Tasks

- Be specific with versions [3fb65d1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3fb65d1a395ca5ec0d510e8422a25f70fde7c465)
- Update to 0.8.8 [2a340de](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2a340de5094279d3f0b1b820af96a8e5d3c03bae)

### Clippy

- Change match to if let [f2af8bb](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f2af8bb3e9939330c4fe3c26e4abaef48560c566)

## [0.8.4] - 2023-10-31

### Bug Fixes

- Also check attempt to vendor if it is also a workspace [c2083f9](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c2083f9423b66f27fec4c41d95af700e8ed6733c)

### Miscellaneous Tasks

- Bump to 0.8.4 [8275784](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8275784921bfc7411d939d1971afbc79800fc4d1)

## [0.8.3] - 2023-10-31

### Miscellaneous Tasks

- Add stop sign in error message [6f12091](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6f1209138aedbd36eb61515e3137386d939c9f9b)
- Bump to 0.8.3 [17e993c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/17e993cd387cfa15454a060c6d77cf547a7406e9)

## [0.8.2] - 2023-10-31

### Bug Fixes

- Also make sure it really has no deps [5f52abf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5f52abf16558c571ded2fcd442234c4ad5a97d71)

## [0.8.1] - 2023-10-31

### Bug Fixes

- Also copy top-level folder if src is a directory [4c0ff68](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4c0ff68f762be3ef08fa331262bd985ce19a775f)
- Also copy top-level folder if src is a directory [bb443ef](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bb443efd009500e010ed91e87c51e1766f404f39)

### Documentation

- Update README regarding the new flag [05649f4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/05649f428300d1320df24489126513b04696ee3f)

### Miscellaneous Tasks

- Update lockfile [8e21d46](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8e21d46f3ac73778fed31c31271ebcc92b335f97)
- Update to 0.8.1 [8f4decd](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8f4decd84a2a139eb09b010027851b7ac7d9eff3)

### All

- Apply clippy lint suggestions [790f85a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/790f85a269958481241e053ef4f0fe69d135396f)

### Clippy

- Apply clippy suggestions [7d13cc3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7d13cc3f6080155e2272458de0d833204c23e896)

### Utils

- Exit vendor step if no dependencies found (#55) [445ae7a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/445ae7add0e7199c2126849e2ec8c5f4cad8690f)

## [0.7.4] - 2023-10-09

### Bug Fixes

- Decompressed tarball may or may not have a top-level folder [4632713](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4632713d93d46dc03bb8c3c56f8b5566bc698372)

## [0.7.2] - 2023-10-07

### Bug Fixes

- Src.clone().src -> actual_src [5ab2288](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5ab228819b24ecd0fee81ff49c6e1a38ce6ed522)

## [0.6.0-alpha] - 2023-09-08

### Bug Fixes

- Change to empty [] list (#30) [41a187f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/41a187fa603334bef39fbe800c8251c3d407ba46)

### Features

- Add cargotoml option. used for syncing (#29) [764c752](https://github.com/openSUSE-Rust/obs-service-cargo/commit/764c752599203a8334ec0645a7f79e2dc082ef8f)

## [0.1.0] - 2019-09-12

### Readme

- Correct OBS links and add attributions (#1) [12b2ff6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/12b2ff6a28cdff11a63530530583bbfeed4de6ba)


