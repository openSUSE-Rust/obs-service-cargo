# Changelog

All notable changes to this project will be documented in this file.

## [4.1.0] - 2024-11-07

### Bug Fixes

- This should pass a valid manifest path-like-string [c484a0d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c484a0dea71ca5e5ce6d10f1f9259cda3620d753)
- This if-else condition is stupid. fixed [a41eb0c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a41eb0ca5e4fd56c558086cad9bc6ba0e18be67e)
- Reintroduce respect-lockfile flag [61d6f40](https://github.com/openSUSE-Rust/obs-service-cargo/commit/61d6f4072926f0e66eca08a5ba2ae3e5d52ea952)

### Continuous Integrations

- Refresh, distro-upgrade, then install [d4f981c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d4f981c50c4c82b48aebe147f7662c7f60996123)

### Documentation

- Update README explaining how respect-lockfile behaves [dad8585](https://github.com/openSUSE-Rust/obs-service-cargo/commit/dad8585d586007cd028116ce4b05e2741061ccf1)
- Include respect-lockfile, and allowedvalue should be allowedvalues [ee551b7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ee551b76c99b6cfc50ac942ce15c35604754408d)

### Improvements

- Use new dependency detection and return properly [71d1ddc](https://github.com/openSUSE-Rust/obs-service-cargo/commit/71d1ddcebc76e240a6c53327739e3736d3caf8fc)
- Fix issues with how we respect lockfile [64090df](https://github.com/openSUSE-Rust/obs-service-cargo/commit/64090df2e37c5dea7670eead60e15b195d37c181)
- Introduce the new dependency detection [8da15a7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8da15a736801bb54d2dad5e71d416b9e50c230df)
- Properly detect dependencies [1dd4b48](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1dd4b48b8472b4716e9afe4da650349ca58aa832)
- Adjust where we pass around the update variable [419299f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/419299f062d41771d4e27b6b6b7d3fcb04b5338c)

### Miscellaneous Tasks

- Update warning message and remove comment [ci skip] [fde3c98](https://github.com/openSUSE-Rust/obs-service-cargo/commit/fde3c981362519408d213aa62773e8d7683b1d5b)
- Make vendor have a separate registry as well [e676ac0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e676ac02ca140c0609d28ce6e2ff5c4819f0f64c)
- Move over the env initialisation to here [bfff7aa](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bfff7aa3d42ad650e0bc4d307c7b113e5a463cf0)

### Other

- Run cargo fmt [ci skip] [33f9016](https://github.com/openSUSE-Rust/obs-service-cargo/commit/33f9016a49394bc296712a548483e9c06ea80c8f)

### Testing

- Readd tests for lockfile hash comparisons between update false and update true [76e8755](https://github.com/openSUSE-Rust/obs-service-cargo/commit/76e8755a337d9dc507e8ab005b8646e3b4152405)

## [4.0.3] - 2024-11-07

### Bug Fixes

- Remove unused import [c0da245](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c0da245edb88fe3f09bf578cb8f2deb476a0514e)
- Locked flag should be passed here [a1a8120](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a1a8120603235c26154639e753f762a6b930c8b2)

### Documentation

- Insert lockfile behaviour in README [949532b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/949532b8b88262e4d2b49e647700ec425d7b9d99)
- Update CHANGELOG [3b98869](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3b988699825e97041c1312a8b924bad6b232b260)

### Improvements

- Address soundness and correctness issues [3e6865b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3e6865bee6015588e6b60baf8aa6148481b811da)

### Miscellaneous Tasks

- V4.0.3 [2db4740](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2db4740b6eaa156a50cc0540f253037c30e6c163)

### Other

- Run cargo fmt [0531da1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0531da14131acd246aaeae8dcffafb5cba20269b)

### Testing

- We have removed this as we lockfile behaviour should also be documented [72ff856](https://github.com/openSUSE-Rust/obs-service-cargo/commit/72ff856e9af69b6df3b08bafa531efff2e7112cf)
- Set the first one to false. [2cec8f6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2cec8f63dc0de9d6a92b8c76c3986fd138d93c54)
- This should not equal to in tests [a1f67ad](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a1f67ad3411e9c46e83357faa6d8497120ff4dec)
- Change from sha2 to blake3 [ccc236d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ccc236d1a590d5ff5eca3f0598384225f11207e5)
- Add tests for lockfile updates [9472614](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9472614caacdf1fe1baf014ce2fa5a0484bb1590)

## [4.0.2] - 2024-11-06

### Miscellaneous Tasks

- V4.0.2 [60e23ad](https://github.com/openSUSE-Rust/obs-service-cargo/commit/60e23adbc2c9fe3edec92d479520052fef16fe8d)

### Refactor

- Check if all manifest needs to get vendored [7b90383](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7b903839b16626af87a7627676a83bda74f54e39)
- Properly cleanup directories and exit if there are actually no dependencies [5c5ff9f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5c5ff9f89b2d3d2254ec7ef99febfa5dbad630a8)

## [4.0.1] - 2024-11-06

### Continuous Integrations

- Remove --release flag on clippy [10d414f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/10d414fdace1de948b13dae525e2953cb6d188a6)

### Dependencies

- Replace sha3 with sha2 [9257586](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9257586eab4990bdda69c103e61242c8343d034f)

### Documentation

- Put the xml service to readme as well [8af1405](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8af1405ed450c8b49e73278f10e49bbc121c88c2)

### Miscellaneous Tasks

- V4.0.1 [aaa627d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/aaa627dd4dc76174c42aed7e8a831341277ad7e4)
- Improve descriptions of each flag [ff12110](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ff121107e24444aa7fa4c77dd2a0730323769241)

### Other

- Fix spaces and tabs and parentheses [2ef0c8b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2ef0c8bf2581043e0627ca847ceefd85b776977e)

## [4.0.0] - 2024-11-05

### Bug Fixes

- If-else to check if it is a workspace or not has been corrected [a532ec2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a532ec2cfc6003874281420ffa73e491394fed41)
- 🎉 squashed all bugs and ready for user testing [b200265](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b2002654242623019d061b17756fdd60dbaa2f38)
- Create the paths first [815afc2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/815afc24270ee43565154865d79d689d29aea8da)
- Change how custom root behaves on "regular" vendor [f643a57](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f643a5793648ce31fae64c305cada693d392f310)
- Rework where we put vendor [6a319b2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6a319b267831a1697e6d87da1730815d29ec20cc)
- Typoed a dash. it was an underscore [b9906cf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b9906cfea1897a21a14e797fbf5bdb5a3d157acd)
- Again, vendor filterer does not support --locked [72c1bd7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/72c1bd7be787149626bb230c108213ac6c872d0e)
- Remove unnecessary if-else that broke the vendor logic [92210b1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/92210b131291a92408d7f4ccf50b7ff7386fb804)
- Added trace feature [1f206d8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1f206d8303a525611284b9efd2934632778c3164)

### Continuous Integrations

- Improve filtering [d99a274](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d99a27472d2c36d53f41d0ab54baf2238ab23593)
- Add paths filtering so that it will only run CI on new code [9ac53b4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9ac53b4f1158b3167bd75139e958c67acdace975)
- Prepare CI test in release mode 🎉 [326397e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/326397e00f3859019250f1b68fffd7d6493d6bce)
- Properly alias the tests instead to run tests in release mode [815d333](https://github.com/openSUSE-Rust/obs-service-cargo/commit/815d33386a106fc45de451c865c0b2e4610fb225)
- Remove --release flag on clippy [6e6d8f2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6e6d8f2cd86a817c869177f43e393f6d060ed2ce)

### Documentation

- Explain how cargotoml behaviour works between methods [330797f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/330797fff30d025b0f08dc26d4f7ccedc7a5bdc0)
- Update README [9f17f57](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9f17f5710c3e3cf8e367423450a3da19caa1b247)
- Update README. outdated statement removed [ci skip] [24181d6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/24181d6d72f79844d35125cca1c57b7aeaad26de)
- Spelling plus make alias clear [c32a870](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c32a870485b580049a76393e9095dbc663fa67a2)
- Fix typo [ci skip] [e47bdcf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e47bdcffc426371e78efc77590fd1a3984f39f44)
- Remove cargo_audit in the examples [ci skip] [cd20772](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cd20772f5eff73e159587a248bc4764c6e326f39)
- Enhance README [ci skip] [928fd7e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/928fd7e61a0d693dcd6409f711ab5dd8bfdd2626)
- Enhance README [ci skip] [915c6c8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/915c6c8f0191df64ef82dc19c4bf35dd5bf8d7c8)
- Enhance README [bd2a7e7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bd2a7e7990c0d3e70ca57872cb2ba187969b4f07)
- Enhance README [1e3aec2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1e3aec22ea907e33927ea523f8d514d09aaef1a9)
- Enhance README [888288c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/888288c8840fab0b4225fce7acc8c7bc9b0cd584)
- Enhance README [0ebe615](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0ebe61529baad85ffa4f7669d1d3b376bc588967)
- Update service file [7fbc366](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7fbc366d4b4118686bce593aacf4c58b1634b03f)
- Update service file [96afe40](https://github.com/openSUSE-Rust/obs-service-cargo/commit/96afe40e5611bf309c721271855fa9c0a61a0a4d)
- Improve explainer about lockfiles [f884fd8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f884fd8531de5ea6f3b82431a286c7803361014b)

### Improvements

- Improve messages [1c08baf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1c08baf676163d8359e7f311af196a5825ae96b5)
- Set env before everything for registry [3da6198](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3da619818576a3808de46fdfc5a407e0b6325187)

### Miscellaneous Tasks

- V4.0.0 [ed3b800](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ed3b800179cdd259f6e5300970491010df6f4178)

### Other

- Check the else clause [784264a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/784264a643010042be70e33d8186a8a6db4180c4)

### Refactor

- Implemented cargo vendor [0fba951](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0fba951e367ccc6945f7e42c33d2695758ea3b00)
- This is a major refactor by combining home registry and vendor. [fdc4981](https://github.com/openSUSE-Rust/obs-service-cargo/commit/fdc4981111be91479c57aa2e3a0bc0c30f18ae1e)

### Testing

- Adjust if-else as without it will lead to failure since we are testing that raw cannot find a vendor tarball [0caf1e6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0caf1e6345a30a7a2e3bb5df049f0bec3685157a)
- Test bonk that it really has no dependencies [a422bc4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a422bc433632065d3a0d19fb4750879445c8ee5b)
- Add test of the output tarball [7289685](https://github.com/openSUSE-Rust/obs-service-cargo/commit/72896850892ea55c20e973f07ec7cde68802044a)
- Update test suite in CI to conform with new code changes [755785c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/755785cccc0d91bf9cb8a5ff5cf53e0f6db1049e)

## [3.6.1] - 2024-11-02

### Bug Fixes

- Forgot to use clap::ArgAction::Set. now finally added [c2fcb05](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c2fcb0520dff165c26737e8bbba976ae92b89898)

### Miscellaneous Tasks

- Release v3.6.1 [f4fb528](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f4fb528dc074e769d52b7e9af69ee02f37566856)

### Other

- Small refactor in generate_lockfile and vendor [be4e87b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/be4e87bfc10c94b6453d42950cded9268b3e40d1)

## [3.6.0] - 2024-11-02

### Bug Fixes

- Used a different cargotoml. now using the correct one for flux [eca8c76](https://github.com/openSUSE-Rust/obs-service-cargo/commit/eca8c768930a842884ac6f613c21c3bfcb9d18d3)
- Rectify mismatched types [cde5b62](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cde5b620dc2881a3f49aaf0ae4583332e8be1742)

### Continuous Integrations

- Install cargo-vendor-filterer from crates.io instead [a34cc71](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a34cc713f58863c7ebe33e0b24025db3130765e0)
- Add dependencies for cargo test [3e6b40d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3e6b40d61ee0d4b2dbd458801fec07aabbdc0801)
- Migrate tests from shell script to cargo [488dec4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/488dec4f9624fb33cbe4f2f3f0f48ba11fc2c4b9)

### Miscellaneous Tasks

- Release v3.6.0 [9c1aa86](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9c1aa86ce5433c6d593ba0f3be1ad925c597aee3)
- Update manifest and dependencies [63ce332](https://github.com/openSUSE-Rust/obs-service-cargo/commit/63ce33287fa379461513bbc973c825f4006d8f96)
- Remove dependencies since libroast is used internally [3ebf4e1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3ebf4e12cc9e7f37a0bb77544e4dbf2797fa71e6)

### Refactor

- Migrates almost all logic to libroast. 🎉 [ce0d23c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ce0d23c2c4080b61ec968b632a50df0823100ec2)

### Testing

- Add tests for cargotoml flag [646fcf6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/646fcf6d98bccf406345bd15c8621c9b875b7ccc)
- Lessen verbosity and rename nonexistent function to existing one [2404269](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2404269fb0ed437431cf1b87849473aee822f1ac)
- Add tests for unfiltered and filtered option [4594a44](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4594a44ff5335b261522139a0f667d502138762b)

## [3.5.4] - 2024-11-02

### Miscellaneous Tasks

- Release v3.5.4 [317b1c4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/317b1c43c26852d577717986c6588f463c536229)

## [3.5.2] - 2024-11-02

### Features

- Add versioned-dirs flag [e81774e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e81774e00b8cfce79ed45d61abb0e1c0233a9832)

### Miscellaneous Tasks

- Release v3.5.2 [618576e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/618576eb9bd1eeb3d3bb2780e4c37f774b46baad)

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

### Other

- Update lockfile and bump crate versions [0dbb45d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0dbb45d0e9b9a4d9b2d4131bec10bdb8a45936d7)
- Improve description [e736094](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e736094899058fb6f04576411713039d35f91578)

## [3.0.0] - 2024-10-15

### Continuous Integrations

- Format check should not fail ci [fa14c74](https://github.com/openSUSE-Rust/obs-service-cargo/commit/fa14c74101c6ed3b1cc047b9b57de87eb3268d47)

### Features

- Support uncompressed tarballs or with `.tar` extension [5733edc](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5733edcf50b494441d8d854a490e0f154934b4f5)

### Other

- Get libroast dep from crates.io [043c050](https://github.com/openSUSE-Rust/obs-service-cargo/commit/043c05053c5db9e5cb697c4aabd80d0f319ec7a7)

## [2.0.0] - 2024-10-12

### Miscellaneous Tasks

- V2.0.0 [d775b7c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d775b7c006f537f704187928d7a53d8418f11e7f)

### Refactor

- Successful move to use libroast [8c2cda4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8c2cda44209556177285ad4de567aa0f7d0bc0d9)

## [1.4.1] - 2024-10-09

### Miscellaneous Tasks

- V1.4.1 [7a2b1cf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7a2b1cf1c81656c51d4ba00fccc72dcdfbfc37c2)

### Other

- Add `--versioned-dirs` flag [1e728a2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1e728a2804ed064e7280697523124381e9c5e0af)
- Use inspect when map returns the original item (#98) [7a62324](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7a623244fa741f1061840f433bc7211e0b3f6674)

## [1.3.6] - 2024-06-06

### Bug Fixes

- Vendor-filterer --all-features rather than --all-features=true (#87) [89b17bf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/89b17bf80fbb416c415a247ea675818506b14b94)

### Documentation

- Fix warning indicator (#89) [6effe77](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6effe77bfdcc4036c4915dc83586b2aed589829d)
- Fix again the warn indicator [837ac22](https://github.com/openSUSE-Rust/obs-service-cargo/commit/837ac221a05c63b8b6fca275c27d66e3554fd708)
- Fix warning indicator [8ef870e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8ef870e9ec4572925af6c694843b133612babb97)

### Other

- Add Pi-Cla (#88) [be74488](https://github.com/openSUSE-Rust/obs-service-cargo/commit/be7448838b011aa07fef649de866fdf533928cb3)

## [1.3.2] - 2024-05-02

### Miscellaneous Tasks

- Update to v1.3.2 [57973e0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/57973e030aef5a841bb8a89dca92c4fb5ca13df1)

## [1.3.1] - 2024-03-30

### Miscellaneous Tasks

- 1.3.1 [0aaa39b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0aaa39b4523163a1eeddbe7202063ec28e302046)

## [1.3.0] - 2024-03-30

### Documentation

- Update CHANGELOG [83e98c2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/83e98c2736fc0b15356f454c3ea4f59133ac0cec)
- Add CONTRIBUTORS.md [0f71a3a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0f71a3afe832f710d76307ac154e8bc87d06157b)

### Miscellaneous Tasks

- Add changelog with git-cliff [ee6cd1f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ee6cd1f926a7041478661c893b3fd9ffba3cef4e)

### Other

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

### Other

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

- We also include build_dependencies [46560c6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/46560c674be831fbd024827b315d0293acdce257)
- Check also the targets section [68d7eac](https://github.com/openSUSE-Rust/obs-service-cargo/commit/68d7eacbaa4ab1eae71eaa1e84931977a5c4b977)

### Documentation

- Put warning on README [ci skip] [7e77a26](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7e77a268caf119a20445335967b221489bf34090)

### Miscellaneous Tasks

- Release 0.8.15 [39f8c58](https://github.com/openSUSE-Rust/obs-service-cargo/commit/39f8c58ed8387f87dc5275f40c5edb07033bb942)
- Nitpick. remove small space [ci skip] [378bbfc](https://github.com/openSUSE-Rust/obs-service-cargo/commit/378bbfc9c7f6026a62c062ea299669d2a4b1fdb3)

### Testing

- Add VBox-Starter in runtests.sh [cb8a9a3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cb8a9a3c432c87ca56b8abb9847b05f7f661669f)

## [0.8.12] - 2023-11-21

### Documentation

- Add possible vendor failures in README [ci skip] [7904a15](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7904a155dfe15cae36b0836db2be8ebd58df0ed8)

### Miscellaneous Tasks

- Bump to version 0.8.12 [f8ec1b4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f8ec1b4d107871a65673842398df8efda0610ca4)

### Other

- Reintroduced tag in README [c67d7d2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c67d7d2f00bb0c07431d4707873c2c954fdedeb0)

## [0.8.10] - 2023-11-18

### Bug Fixes

- Check if dependencies section is empty or not [621dea1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/621dea1f84cd37f7a178bf5878c0e22354961c6b)

### Continuous Integrations

- Put the tests into a convenient script [84cca41](https://github.com/openSUSE-Rust/obs-service-cargo/commit/84cca411fbf4b632bbc6b6eb16b19876154882f2)
- Install libzstd. it's needed [f7039e3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f7039e3c078c5c88f843712e8f4b4292c12df360)

### Documentation

- Update README [ci skip] [4c15d8e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4c15d8e6f7dac07267d558199e71bb6939aeb75b)
- Add status badge in the readme [06b24c0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/06b24c0c733c7d08bf7e838889a0b393f1ec85f8)

### Improvements

- Better handling on when and when not to vendor [751775c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/751775c2c070481c9a755fa86faeb12c503d8729)

### Miscellaneous Tasks

- Remove redundant messages [d0cbcd4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d0cbcd437c9f604566ac3fba6923ea6b8d14b8a4)

## [0.8.9] - 2023-11-17

### Bug Fixes

- Sometimes lockfiles can't be found [8b9ef3f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8b9ef3fec060927deaa57c67191a01e036d8b825)

### Miscellaneous Tasks

- Ready for patch release 0.8.9 [bf1ce2c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bf1ce2c3ce16a75b040685d7c913358b432266b4)

## [0.8.8] - 2023-11-17

### Bug Fixes

- Just warn and don't return an error [b95565f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b95565feed61bf4107fd45baee33d44d50879dbb)
- Hasdeps should inherit should_vendor value [696a35a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/696a35a909605f0b059d2974aaed30b11d500b37)
- Also join the paths from `prjdir` [ec3b628](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ec3b628c2de748e2f5600a0d36d7284d3e968f48)
- Ensure `basepath` and sources are joined as one path [6f39308](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6f39308f0c7e814f6ab6e345f79f25199747eb73)

### Documentation

- Add dependencies section in the README [9609059](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9609059b9357e9b0d449539da5d8abaf00d8f1cc)

### Miscellaneous Tasks

- Update to 0.8.8 [2a340de](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2a340de5094279d3f0b1b820af96a8e5d3c03bae)
- Be specific with versions [3fb65d1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3fb65d1a395ca5ec0d510e8422a25f70fde7c465)

### Other

- Change match to if let [f2af8bb](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f2af8bb3e9939330c4fe3c26e4abaef48560c566)

## [0.8.4] - 2023-10-31

### Bug Fixes

- Also check attempt to vendor if it is also a workspace [c2083f9](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c2083f9423b66f27fec4c41d95af700e8ed6733c)

### Miscellaneous Tasks

- Bump to 0.8.4 [8275784](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8275784921bfc7411d939d1971afbc79800fc4d1)

## [0.8.3] - 2023-10-31

### Miscellaneous Tasks

- Bump to 0.8.3 [17e993c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/17e993cd387cfa15454a060c6d77cf547a7406e9)
- Add stop sign in error message [6f12091](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6f1209138aedbd36eb61515e3137386d939c9f9b)

## [0.8.2] - 2023-10-31

### Bug Fixes

- Also make sure it really has no deps [5f52abf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5f52abf16558c571ded2fcd442234c4ad5a97d71)

## [0.8.1] - 2023-10-31

### Bug Fixes

- Also copy top-level folder if src is a directory [bb443ef](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bb443efd009500e010ed91e87c51e1766f404f39)
- Also copy top-level folder if src is a directory [4c0ff68](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4c0ff68f762be3ef08fa331262bd985ce19a775f)

### Documentation

- Update README regarding the new flag [05649f4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/05649f428300d1320df24489126513b04696ee3f)

### Miscellaneous Tasks

- Update to 0.8.1 [8f4decd](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8f4decd84a2a139eb09b010027851b7ac7d9eff3)
- Update lockfile [8e21d46](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8e21d46f3ac73778fed31c31271ebcc92b335f97)

### Other

- Exit vendor step if no dependencies found (#55) [445ae7a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/445ae7add0e7199c2126849e2ec8c5f4cad8690f)
- Apply clippy lint suggestions [790f85a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/790f85a269958481241e053ef4f0fe69d135396f)
- Apply clippy suggestions [7d13cc3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7d13cc3f6080155e2272458de0d833204c23e896)

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

### Other

- Correct OBS links and add attributions (#1) [12b2ff6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/12b2ff6a28cdff11a63530530583bbfeed4de6ba)


