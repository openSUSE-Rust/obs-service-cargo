# Changelog

All notable changes to this project will be documented in this file.

## [7.1.0] - 2025-08-01

### Bug Fixes

- Forgot to set clap::ArgAction::Set for filter and versioned-dirs [c26db9f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c26db9f556d99d5188f5968055869f383a356027)

### Dependencies

- Update libroast to patched version 10.0.1 [5985ee1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5985ee1ceb41d11a33b1ab0b4217068095a22d64)
- Update libroast to v10.x [0654e2e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0654e2efa5d34376b95d6033a30f1b4fc1c2c218)

### Documentation

- Improve README.md [f492aff](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f492aff6b70ec15c54c50bd1800062bdac7c6b0e)
- Improve README.md [ca111de](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ca111de6fe77845aff46d63d4add65dfa599cf3e)

### Miscellaneous Tasks

- Update dependencies [785a5d6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/785a5d6bf86e8c8c6b0cfb9828fd78ce764c2eb5)

## [7.0.1] - 2025-06-25

### Documentation

- WARN -> WARNING [7db8aad](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7db8aad8dc83075ce4550eca4ebbcc3f3ea1a86a)
- Improve README.md [6bef49e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6bef49e5743debc5ec165dd08a83542428b277c5)

### Miscellaneous Tasks

- Bugfix release v7.0.1 [bcae1f1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bcae1f1af21eceed4eb8c26234383b9fd10d809a)

### Testing

- Change tokio spawn logic to perform batched tasks [c867a35](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c867a357d9fca1f8ff596ece00a4dfbd687feb49)

## [7.0.0] - 2025-06-22

### Dependencies

- Bump all available updates/dependencies [f348ea8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f348ea821c709522b09a2535c007f9ecfaa84093)
- Update libroast to v9.x [6cd9c17](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6cd9c171de7b3ee8ef1f224b1662285ddc52d45b)

### Miscellaneous Tasks

- V7.0.0 [700bd50](https://github.com/openSUSE-Rust/obs-service-cargo/commit/700bd506461e963f1dc33d09166ee3d8d227c6fe)
- More cleanup [08c5101](https://github.com/openSUSE-Rust/obs-service-cargo/commit/08c51016058472a29bb699f41a5fabdaa8b738d3)
- Cleanup code [0d2c3a6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0d2c3a69206b7691f41d6bcfd602ad58fb8822ef)

### Refactor

- Adopt to libroast v9.x API [cdd35e3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cdd35e3c6a3b748548c0b8cf79da70faa22e1525)

## [6.0.9] - 2025-06-19

### Bug Fixes

- Bring libroast fix where we just add one newline [35e3c5f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/35e3c5f517c69aba5de9f066588094bea92fbd3b)

### Miscellaneous Tasks

- V6.0.9 [bf21ecd](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bf21ecd2f4eaea3cffe8b6fcdac8912da7efba3a)

## [6.0.8] - 2025-06-19

### Bug Fixes

- Update libroast to v8.1.4 [df6aa3b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/df6aa3b5c3ca2aa9227606eebc64d7d2804bc7e7)

### Miscellaneous Tasks

- V6.0.8 [d505ce7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d505ce7bd6b180c49a2638229ddac1e98c74847a)

## [6.0.7] - 2025-06-19

### Bug Fixes

- Apply clippy fix for `redundant-field-names` [52405a8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/52405a8e697950364498c1237748fc3ed6afe81f)
- Apply clippy fix for `field-reassign-with-default` [2237e16](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2237e16a2e9a5658fafce8a7d3defb0208d0e647)
- Wrongly used clap derive attributes for requires. rework VendorArgs as well provide defaults. [516b3e7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/516b3e7678c192ddb6dccedb4ce9fb0b91a184c6)
- Bring in new version of libroast by running `cargo update`. [dd4c925](https://github.com/openSUSE-Rust/obs-service-cargo/commit/dd4c92565e77ee446f29e7e0ed08efbef0b2e01e)
- Apply clippy fixes [7574d42](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7574d42f03979e0fd51ea352b48d49d8afd62117)
- Only provide a warning. `--no-root-manifest` is never used in `vendor.rs` [f9a9e55](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f9a9e55fcf18d4c44b1e24fce3ebfe33bbd5440c)
- Make `no-root-manifest` field to `Option<bool>` [1924212](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1924212999fbaa6072a96c39be12511754091cd2)

### Miscellaneous Tasks

- V6.0.7 [658145b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/658145bf5c371271e78212e7605d89b7dc45bf24)
- Apply new Option<VendorArgs> change [8bcdc62](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8bcdc6298539a7c417dcfa1b24304403c3c13844)

## [6.0.6] - 2025-06-14

### Bug Fixes

- The `target` field for build targets is now being used [c10b125](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c10b125b3968257754f9bc4b526198105b2c94a3)

### Miscellaneous Tasks

- V6.0.6 [dc00b3c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/dc00b3c7862b33121c67bc51b86a5fad38526c3f)

## [6.0.5] - 2025-06-13

### Dependencies

- Bump libroast to v8.x [347c96f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/347c96f63979549c23923841dbd25a5dbcca5f7d)

### Miscellaneous Tasks

- Bump to version 6.0.5 [10b4f62](https://github.com/openSUSE-Rust/obs-service-cargo/commit/10b4f62b994ff1a3670f955509888bde044f6dba)

## [6.0.4] - 2025-06-08

### Bug Fixes

- Error message caused confusion here. changed it so that it passes any general kind of errors. [11e3c0a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/11e3c0ab99b12832cd2e69c2cde565c06df398c3)

### Dependencies

- Adjust libroast version and update dependencies. [e6e7395](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e6e73950b0802315ff83e11dc21df673adef73f8)

### Documentation

- Update cargo_vendor service documentation [d6be731](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d6be731aa98a3603236a5b8b3ed60cbedd895c01)
- Update README [8c95c36](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8c95c36be96c4214d7f70270e78d16fb42c6e7d2)

### Miscellaneous Tasks

- Bump to version 6.0.4 [b8afe81](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b8afe8113610d2a4a1dc5c59cb8009df1265e3ed)

## [6.0.3] - 2025-06-03

### Bug Fixes

- Bring in libroast fixes from newer version [d01e674](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d01e6745902e5085d6a7b3a3972fa143ab9ef43a)

### Miscellaneous Tasks

- Bump to version 6.0.3 [d2d06f5](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d2d06f5e44770ee4e55282b83d33b2fc8ea1b589)

## [6.0.2] - 2025-06-03

### Bug Fixes

- Changelog generation is fixed in libroast 7.2.2 [a9dea69](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a9dea69555bee8b2b855f2abde34736692416890)

### Continuous Integrations

- Leap does not have gzip in its image by default [96601a0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/96601a0add20b2fc3f937b84ac02583b303e6875)
- Leap does not have tar for some reason in its docker image. [5420a56](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5420a561d74250e3022ba997601dd70e6ac9cbd4)
- Run CI if workflow changes [cc328c1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cc328c13fcd2cb1e24fa3f9a8b3ec7d8c9416bdd)
- Change to leap container [032e85f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/032e85f8f1394e81463ab25f7fe2fe60fe6a0010)

### Miscellaneous Tasks

- V6.0.2 [a14848a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a14848acab466bc1bb4bfe32e6d42c04992c7f26)

## [6.0.1] - 2025-06-02

### Documentation

- Behaviour changed. i don't want TMPDIR to be filled up like crazy. [1a41a5a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1a41a5a269bcfba2dcbfa46a0ae6b7e625d2eedb)
- Update README.md [770de72](https://github.com/openSUSE-Rust/obs-service-cargo/commit/770de723ee30ed80478d72915ac6d709fdf388b8)

### Improvements

- Just pass the current workdir instead [1ceb0d1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1ceb0d1938542e5fae25a22364ccab00ea21c97c)

### Miscellaneous Tasks

- V6.0.1 [b1c1db4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b1c1db47429ae1d7caefe2a5e08f829b89c9f246)

## [6.0.0] - 2025-06-01

### Bug Fixes

- Revision should be an `Option<String>` in obs service cargo [2714f44](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2714f44dfc6b945bc36da486e377025647924a3c)
- Clippy lint fixes + adding missing fields in tests. [a728118](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a728118e6578e0674e1c6fd211a3fad6fcf3594d)

### Dependencies

- Update libroast to 7.1.2 [b3519a0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b3519a01be92bfd7884931e1e17e4f017e1c1da7)
- Update libroast to v7.1.1 [559cc66](https://github.com/openSUSE-Rust/obs-service-cargo/commit/559cc66b8928c48530a0245cbe6eac16dffaa78a)
- Update libroast to v7.1.0 [1dc6072](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1dc60723a662de5c42f9be0c042631231cbf1b44)
- Update Cargo.lock [4f76cd5](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4f76cd5fae18deacd4151b748e74c4210572924e)
- Only use git sources for libroast [f0dbc9a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f0dbc9a124eb1e7a23faf92a1559c292b80a80ff)
- Get libroast from registry now [4fa7b42](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4fa7b42dcc299906fd079edec8d87d59b16714e9)
- Use another fixed change in libroast [3100c6c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/3100c6cc90826436aabb1a53fd5e92ab46992870)
- Add url crate [b7243a1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b7243a177efca4cd11910e572dac955f85bec653)
- Use another fixed change in libroast [4108601](https://github.com/openSUSE-Rust/obs-service-cargo/commit/41086012822a661ee474494f3e1cfe696aac1eee)
- Update libroast to the working commit [fec6956](https://github.com/openSUSE-Rust/obs-service-cargo/commit/fec6956748bace0dd830d5cfbb21f2799d7f2fb8)
- Use git source at a specific commit hash for libroast. [d0b8592](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d0b8592886d0c2494d4ed80e792ad7ed901f36a5)
- Bump libroast to 6.1.0 [6d3fd1f](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6d3fd1f3f8d9c51751dc301e46988e7079698c05)

### Documentation

- Update service file definition [4cf1c67](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4cf1c678a4c04a69a8a644b66f975a2bc0d34224)
- Update wording in code docs [145161b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/145161bc0cfc80e6139ec2c85dea45f688baf7b9)
- Update doc comments to match intention [b080951](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b080951456d9a4d4a3d9429382dbf2f9a882690d)

### Features

- Roast_scm + obs-service-cargo_vendor in one go [1dcb937](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1dcb93798dee90330aaded26b601a8b20e117b50)

### Improvements

- Port over roast scm specific fields to obs service cargo [8fbc6af](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8fbc6af21e2349ac585cc54fdfb40ad2bbdab56b)

### Miscellaneous Tasks

- Bump to v6.0.0 [bfb4b70](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bfb4b70af4219d27cdd038b13fb90621b375e7ca)
- Apply clippy suggestions [94d47ce](https://github.com/openSUSE-Rust/obs-service-cargo/commit/94d47ced0e125cde8ce6a6a8bbba5a425c6151ef)

### Other

- Run `cargo fmt` [f976cc2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/f976cc250041cb89f1c9ab6db92ec7e01becdda0)

### Refactor

- Change this field to `String` instead of `PathBuf`. [eeaf4e7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/eeaf4e735956f52e71feeb14836594cd3b461ba2)

### Removed

- Woah this test file has not been removed since lol [eaa15d9](https://github.com/openSUSE-Rust/obs-service-cargo/commit/eaa15d9c54a8521d562ec693269c31b27e7cb531)

### Testing

- Update bonk to v0.4.0 [2fedf31](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2fedf31e2818f7b4ed2bd2e562b53dacfc4b4b63)
- Ignored -> ignore [83be42c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/83be42c64b2e12a838e57b7061b13a0ac902d9aa)
- Fix tests by ignoring those that change directories in the environment [d547eba](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d547ebafc2eaecb2ec201295bfb0d75fdd26125c)
- Add tests that points to a URL [0f475b2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0f475b281506181ada2903021f764d603adf1e1e)
- Add test file [4dbcf04](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4dbcf04af486eff0eb5578c5221152ceaef511c4)
- Update struct fields of `Opts` in tests [322ece6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/322ece6fb3e09167dd99fe1fd07b4d52168e1d4a)

## [5.1.0] - 2025-04-13

### Continuous Integrations

- Run also when lockfile is updated. [b65b6a3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b65b6a3e26f163b4d6fb54dfca64dd5cd8980857)
- Set TOOLCHAIN_VERSION to stable [d972c41](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d972c41963b49277ef9d6d11e40ac7ca8d29397d)

### Miscellaneous Tasks

- Bump to v5.1.0 [2910335](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2910335f66158a658dfeebca8b8bf6cfc09ba1c0)
- Remove unnecessary newlines in Cargo.toml configuration. [e918141](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e918141025a8e6492109e760f8a25231fce822b2)

### Other

- Apply fix for "double_ended_iterator_last" clippy error here too [c30a632](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c30a632619be31030a43a255b5bd529e5f359ce9)
- Apply fix for clippy error "double_ended_iterator_last" [afad759](https://github.com/openSUSE-Rust/obs-service-cargo/commit/afad759ca3a7f83ec62ace8de68dafdf754a0668)
- Update Cargo.lock [5b5c3d7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5b5c3d768446c799227a0edd9fcb7a7a3f9cd528)
- Always select stable [7acbe36](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7acbe3675136e151a9397b55153a0c21fdc15e6f)

### Testing

- Make `segments` mutable [e7e7ed2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e7e7ed2e469781518223f274f64fa98e72d916fe)

## [5.0.0] - 2025-03-02

### Miscellaneous Tasks

- V5.0.0 [dcd5ba7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/dcd5ba7e9fa986932794c4e0f8b3fdd81f17173b)
- Bump deps [af0b3f8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/af0b3f86c11d655c720d775d011c8f57c4a02199)
- Update MSRV to 1.85 and Rust edition to 2024 [93fb1a3](https://github.com/openSUSE-Rust/obs-service-cargo/commit/93fb1a386168ff336ed9caaac0d6c7f99f9e152b)

### Other

- Update Cargo.lock [1ea9031](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1ea9031c61bc05033040a129413bd7920422f785)
- Trying to patch gix-worktree-state [c3dea6a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c3dea6af517b18f602949f95bee3332bb8f701e5)

### Refactor

- Update code to Rust 2024 edition [0c65f29](https://github.com/openSUSE-Rust/obs-service-cargo/commit/0c65f296e42415ccbca6290f7494a12272a7103d)

## [4.5.0] - 2025-01-30

### Bug Fixes

- Missing last double quote now added. [195e00a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/195e00aa011c5e422c693ffe24da9ee3a2c64054)

### Continuous Integrations

- Install rust with \`rustup show\` command [b7afcc6](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b7afcc631019cf9d4dc5fac4e4eb536fde141582)

### Miscellaneous Tasks

- V4.5.0 [fa09f56](https://github.com/openSUSE-Rust/obs-service-cargo/commit/fa09f56ed1ecc2f9295a6c585da0025abab394ba)

### Other

- Add most used components for rust development [ed163f0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ed163f00b53da431e4d10f840af1183aaa87cf4c)

### Security

- Update gix-worktree-state to 0.17.0 [1fda083](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1fda08353f006c8f109d8673d25ffe0ff446e118)

### Testing

- Update sources for roast. [92ed03e](https://github.com/openSUSE-Rust/obs-service-cargo/commit/92ed03e61301f922ccaad933e714bb9b88575693)

## [4.4.1] - 2024-12-20

### Bug Fixes

- Avoid infinite recursion causing stack overflow [411432a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/411432ae8efc9e900720027523f5e05c3674bd74)

### Miscellaneous Tasks

- V4.4.1 [a3c37da](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a3c37da9ebdb588046680e914a79b6ef90024d7c)

### Testing

- Add just to the test vendored files [9b8a085](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9b8a085ad6c72152aef48cba2f15282cfd23f580)

## [4.4.0] - 2024-12-19

### Bug Fixes

- It should be using custom_root and not setup_workdir [d742d0c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d742d0cb94b39119de976514fa4134ab76950e3a)

### Improvements

- Ensure paths are handled. also output the hashes of lockfiles before and after. [4f41305](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4f41305c7b1619c35931d31b7505648b35974859)

### Miscellaneous Tasks

- V4.4.0 [b0bcb9b](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b0bcb9b4fd71a1d7b789cb8d8ebe39a85ca6e039)
- Apply clippy suggestions [a760d59](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a760d5988b90a938111db3396bbf7caead42bae3)
- Set MSRV to 1.83 [5430b09](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5430b093fc5f6d4ab2ae31a8fabdd34bbd0691c7)
- Update Cargo.lock [5c50afd](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5c50afd7c57b9ef0deb698bb78a5e9a4c814086d)

### Testing

- From_str to from in PathBuf [676d402](https://github.com/openSUSE-Rust/obs-service-cargo/commit/676d402b275d5264092867ec7488901ea53e6aa1)
- Add tokenizers for registry method testing [68243ec](https://github.com/openSUSE-Rust/obs-service-cargo/commit/68243ec9a71e5ce8b9867fa4dec3e3997513b5d8)

## [4.3.6] - 2024-12-08

### Improvements

- Ensure to communicate well to users when a "No space left on device occurs" [951b67a](https://github.com/openSUSE-Rust/obs-service-cargo/commit/951b67a6ea66f0aae510d4c82ac768af5af972d5)

### Miscellaneous Tasks

- V4.3.6 [6f527e8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6f527e894816f3a8428b8aa3c4cf1a84cee109b0)

## [4.3.5] - 2024-11-27

### Bug Fixes

- Finally added the alias. 🫠 [6edd986](https://github.com/openSUSE-Rust/obs-service-cargo/commit/6edd986a6f2b2c1b94fd700bd3002d88904dbab1)

### Miscellaneous Tasks

- V4.3.5 [b439a64](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b439a648b092b9927c35f18b5e60b246e80f7f8b)
- V4.3.4 [5534b17](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5534b1754c9d2c89023a48d8b69cbf1dc06b7baf)
- Update roast to 5.1.7. update other dependencies as well. [1f83e79](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1f83e79c5992717a80d0abbd36e487701070a356)

## [4.3.3] - 2024-11-17

### Bug Fixes

- Let cargo just put where the vendor directory is by default which is the curdir [4facec0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4facec0ef23a074763bd12fe5d831cbff52af588)

### Improvements

- If workspace, pass the workspace flag in cargo update [ccfcfd4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/ccfcfd4cb247d00fe78a47bb80fbec648993bb4b)

### Miscellaneous Tasks

- V4.3.3 [bde0300](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bde030067fa40e61894d89082e9b2a348700efb2)

## [4.3.2] - 2024-11-17

### Bug Fixes

- Updating specific crates was not properly implemented. now resolved. [4ce3473](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4ce347325b6b25587b852fa3c1e64ac29b8ab7d0)

### Miscellaneous Tasks

- V4.3.2 [5efa1c7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/5efa1c79a626eb4436c678829f5724e024fcf804)

## [4.3.1] - 2024-11-17

### Bug Fixes

- Correctly use setup_workdir and custom_root [e8b84fd](https://github.com/openSUSE-Rust/obs-service-cargo/commit/e8b84fdbf36e1e5aec6f2398cd71b674b6f6337a)

### Miscellaneous Tasks

- V4.3.1 [add2cd1](https://github.com/openSUSE-Rust/obs-service-cargo/commit/add2cd140d56bf452b4dae5e62e141cbacac0a3b)

## [4.3.0] - 2024-11-17

### Bug Fixes

- Resolve location for the vendor directory [d075067](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d075067848d1dc419650ce710a36e59e631dd61c)
- Do not canonicalize in first_manifest [9391bbf](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9391bbf9f522059b36963bac602b0a7fa76b00a3)
- Remove the wrongly used else-clause [9b599a0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9b599a07ce3df2fa5112aca4f6bb2d057d6b5da6)

### Documentation

- Update docs to include the new feature with feature additions [9fbb7e5](https://github.com/openSUSE-Rust/obs-service-cargo/commit/9fbb7e55d21a6a705b854959e39d9fca02a5d852)

### Improvements

- Ensure that the additional manifest paths ends with `Cargo.toml` [37a9d48](https://github.com/openSUSE-Rust/obs-service-cargo/commit/37a9d48d0974b1240e086a93ce5cfd31134a706d)
- Parent path of Cargo.toml files are now set as current directory when invoking commands [d3354d8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/d3354d8cc1b806a19a7b6a7dee94754aae913611)

### Miscellaneous Tasks

- V4.3.0 [36a5942](https://github.com/openSUSE-Rust/obs-service-cargo/commit/36a59427cedbeab67f6166e047d0df93919fd5d1)

### Removed

- It's not our bug. using `--target` for multiple triples now removed [8c33c19](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8c33c19572a0977a776c6efda012540cd28609b4)

## [4.2.2] - 2024-11-16

### Bug Fixes

- Add more info [1b76898](https://github.com/openSUSE-Rust/obs-service-cargo/commit/1b76898784a4ef099fb5886c9750a60d4047d5d3)
- Add warning [18a17da](https://github.com/openSUSE-Rust/obs-service-cargo/commit/18a17da069998e8d5e4e7b7881edeaab83f2fd01)

### Miscellaneous Tasks

- V4.2.2 [bb7019c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/bb7019c08fa6ac1b38e58c1a5cd275a3651f4a53)
- Update crate version [cfa227d](https://github.com/openSUSE-Rust/obs-service-cargo/commit/cfa227d748227c6d47e660bc7cd84a089f51c5b1)

## [4.2.1] - 2024-11-16

### Bug Fixes

- Ignore error when crate dependency does not exist for manifest path [60f04a4](https://github.com/openSUSE-Rust/obs-service-cargo/commit/60f04a44a043556f929ea93c8269008e1ada931d)

### Miscellaneous Tasks

- V4.2.1 [8b411db](https://github.com/openSUSE-Rust/obs-service-cargo/commit/8b411db17dc4008f4eb4dbde51366773de595b64)

## [4.2.0] - 2024-11-16

### Bug Fixes

- Can now determine a member that uses a glob pattern [2e9bad2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/2e9bad24aac2846a5bc0557c55adec3bacccee97)

### Documentation

- If set to true [c9fb0c2](https://github.com/openSUSE-Rust/obs-service-cargo/commit/c9fb0c253dbfe3d4fc8d16b27be4306f89408d09)

### Features

- Update can now specify recursive or precise [b4e8525](https://github.com/openSUSE-Rust/obs-service-cargo/commit/b4e852538ed5fe29c9d7b1cc7e5fb1dbd6b15941)
- Add all target triples as no opt for cargo-fetch [a34992c](https://github.com/openSUSE-Rust/obs-service-cargo/commit/a34992cbf696150573cd89e2a0e607b1f2380a45)

### Miscellaneous Tasks

- V4.2.0 [4109120](https://github.com/openSUSE-Rust/obs-service-cargo/commit/410912084d973597bc9fb78ef59778cfae90fdc5)

## [4.1.2] - 2024-11-08

### Bug Fixes

- Correct error message for lockfiles [176a5f8](https://github.com/openSUSE-Rust/obs-service-cargo/commit/176a5f83598dfe23b92f281c7c921c65e31cd999)

### Documentation

- Replace underscore with dash [4e2ba58](https://github.com/openSUSE-Rust/obs-service-cargo/commit/4e2ba58534dabdcdd45923bb4804d231c0fea823)

### Miscellaneous Tasks

- V4.1.2 [72eae19](https://github.com/openSUSE-Rust/obs-service-cargo/commit/72eae19a79db12b1cfce4fe182f591cca494017d)

## [4.1.1] - 2024-11-08

### Miscellaneous Tasks

- V4.1.1 [7855a85](https://github.com/openSUSE-Rust/obs-service-cargo/commit/7855a85d41ba1f9bf5b6df8dbec211a71ea1cf1d)

### Other

- Remove unnecessary if-elses. [59f70c0](https://github.com/openSUSE-Rust/obs-service-cargo/commit/59f70c032c138fea9e203caec7c499ffc1380cf3)

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

- V4.1.0 [09ef9c7](https://github.com/openSUSE-Rust/obs-service-cargo/commit/09ef9c71bf6049a1ca7e4f2f13e65d6ac1040660)
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


