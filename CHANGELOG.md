# Changelog

with help from [`git log`](https://www.git-scm.com/docs/git-log):

```shell
git log --oneline --format="- [%h](https://github.com/ahdinosaur/nest/commit/%H): %s"
```

## [nest-cli-0.3.0](https://github.com/ahdinosaur/nest/tree/nest-cli-0.3.0)

- [1a2ae4f](https://github.com/ahdinosaur/nest/commit/1a2ae4fa0dd7b4e5b3af3fc01758d43df81caf1e): update nest-cli to use nest@1.0

## [nest-1.0.0](https://github.com/ahdinosaur/nest/tree/nest-1.0.0)

- [22dfbe2](https://github.com/ahdinosaur/nest/commit/22dfbe2c0b7dad5e96b89345a8ecb5ce545f9751): handle the fallible conversions gracefully!
- [8ef2949](https://github.com/ahdinosaur/nest/commit/8ef29492427a19fff81d9b753f0dea3e464a00d5): move all functionality for each source type into a single file
- [799e8c3](https://github.com/ahdinosaur/nest/commit/799e8c3fe9f91a0a8f96d73563870f96511685b1): easier type trait syntax
- [b67a0e3](https://github.com/ahdinosaur/nest/commit/b67a0e3515da653ee55af03833ff1944cba8fb75): even more explicit types using trait placeholders
- [6af4a59](https://github.com/ahdinosaur/nest/commit/6af4a59afd30244724df49e245c2b3055e8fa0aa): move value conversion up a layer
- [50da22a](https://github.com/ahdinosaur/nest/commit/50da22a119f5d1b78a60796932b41edf98efb4ca): more info in serde errors
- [263f17b](https://github.com/ahdinosaur/nest/commit/263f17bf6b12cdcd18cf8043db54346dd5e4fd10): as you wish clippy
- [bdd77a3](https://github.com/ahdinosaur/nest/commit/bdd77a3f3cfccc8aaa7c0bb9d0d2f2e37a124b3e): own all the objects!
- [582c737](https://github.com/ahdinosaur/nest/commit/582c737b7f8a7c4f52b80cbc30b14dd798f243f3): i'm over references
- [e81d41e](https://github.com/ahdinosaur/nest/commit/e81d41ebd11ca33af442e2cf029a4de4508d33f3): so close yet so far
- [b7eec16](https://github.com/ahdinosaur/nest/commit/b7eec16398d9707482f8164d5e7f4de652161ae6): checkpoint
- [475f83e](https://github.com/ahdinosaur/nest/commit/475f83e28d789a56c1f42a6813d3befe45dc051e): preserve order in objects
- [2f5bb86](https://github.com/ahdinosaur/nest/commit/2f5bb86b41ebc419bbd0e73a84fead28c7c66e20): move Path into separate module
- [4cff7cf](https://github.com/ahdinosaur/nest/commit/4cff7cfb29ab6203f090bb8d3f30bca0fa14df15): fix tests using type ascription for &[&str; 0]
- [0cad9ed](https://github.com/ahdinosaur/nest/commit/0cad9ed9736f3c34b5dff28f29b814552be9bdd1): yay StorePath works
- [402049c](https://github.com/ahdinosaur/nest/commit/402049cc0f57431b806941e6aa1969742731874d): try to create StorePath struct
- [60a722b](https://github.com/ahdinosaur/nest/commit/60a722b36b8a34858e52515b09bbcad50ca30e3c): discover TryFrom and TryInto traits, yay!
- [06626f6](https://github.com/ahdinosaur/nest/commit/06626f6e5fde7f35de2ecd6ae88cbc1fac853c4b): add support for toml source files
- [d5b063e](https://github.com/ahdinosaur/nest/commit/d5b063e3aef8cd8678e3f0a1c9a3f9cf6c64d0f8): add support for yaml source files
- [d3169da](https://github.com/ahdinosaur/nest/commit/d3169da66221f8b8282117e00ff7ca92edaadb65): Revert "try to fix Into<Result<T>> but yeah nah"
- [3da095e](https://github.com/ahdinosaur/nest/commit/3da095e5a0c2ae369ce127968887b3b9487a3f35): try to fix Into<Result<T>> but yeah nah
- [1bcacce](https://github.com/ahdinosaur/nest/commit/1bcacce1e95c561e0d59313ffa575a220882153d): split source format code into separate files
- [317288a](https://github.com/ahdinosaur/nest/commit/317288a295caddfcd1a19578a928ccf0b801af5b): use objekt library to handle cloneable trait object madness
- [e4b65f0](https://github.com/ahdinosaur/nest/commit/e4b65f00f2f28a2ebdb021f0e8baf5ca56908c6a): refactor schema leaves as sources, using traits
- [de3d32b](https://github.com/ahdinosaur/nest/commit/de3d32b766b90bbc391cf6802b1cc5eb57087205): nest: add support for .hjson format
- [cfd7745](https://github.com/ahdinosaur/nest/commit/cfd7745b03ec535e86166b5d054af33fae787d36): nest: don't use serde_json::Number as Value::Number
- [08618f8](https://github.com/ahdinosaur/nest/commit/08618f86f705e22e03829af7c9ddb02afc003bc6): satisfy the cargo clippy cult
- [3bf3209](https://github.com/ahdinosaur/nest/commit/3bf3209a65456946e79717371801fc6b7755162d): cargo fmt --all

## [nest-0.4.0](https://github.com/ahdinosaur/nest/tree/nest-0.4.0)

- [090c3ef](https://github.com/ahdinosaur/nest/commit/090c3ef34f941005d8f1948f81ed7781b90a5660): add trailing newline to written file
- [3a4b419](https://github.com/ahdinosaur/nest/commit/3a4b4196fac1b902d1349f795cbb579eef71e629): show the example-data better
- [829d061](https://github.com/ahdinosaur/nest/commit/829d0615debc28b94aa00508baf4e6e2dbcd3d0a): add nest-cli usage to README
- [413445b](https://github.com/ahdinosaur/nest/commit/413445bce15fdd49085bf0312ec47aaf01f7234b): fix the emoji

## [nest-cli-0.2.0](https://github.com/ahdinosaur/nest/tree/nest-cli-0.2.0)

- [6a528b4](https://github.com/ahdinosaur/nest/commit/6a528b467e31c93758847fb44485b0b6f94b6989): bump nest-cli to use nest@0.4

## [nest-cli-0.1.1](https://github.com/ahdinosaur/nest/tree/nest-cli-0.1.1)

- [721267d](https://github.com/ahdinosaur/nest/commit/721267ddaa0049c54365586593abbccb958bde70): bump nest to 0.3.6
- [805d6ae](https://github.com/ahdinosaur/nest/commit/805d6ae489aa25c5a227b3a5722a5a3bc0ca1be3): better docs for both modules

## [nest-0.3.6](https://github.com/ahdinosaur/nest/tree/nest-0.3.6)

- [805d6ae](https://github.com/ahdinosaur/nest/commit/805d6ae489aa25c5a227b3a5722a5a3bc0ca1be3): better docs for both modules
- [852c584](https://github.com/ahdinosaur/nest/commit/852c5846aeb619ef249bfc4fd3d7eb3f6ee37242): minor fixes to nest

## [nest-cli-0.1.0](https://github.com/ahdinosaur/nest/tree/nest-cli-0.1.0)

- [34b28cb](https://github.com/ahdinosaur/nest/commit/34b28cb42d4df79f92fdf4f7725cf9c09a6477ea): ready for nest-cli pre-release
- [2137ca5](https://github.com/ahdinosaur/nest/commit/2137ca55ce5a431e58808101158efb09603828ca): start working on nest-cli

## [nest-0.3.5](https://github.com/ahdinosaur/nest/tree/nest-0.3.5)

- [bbea6d7](https://github.com/ahdinosaur/nest/commit/bbea6d7dd51c317664c78df31234fb8b529db0f5): yet another failed test with fix

## [nest-0.3.2](https://github.com/ahdinosaur/nest/tree/nest-0.3.2)

- [ba31986](https://github.com/ahdinosaur/nest/commit/ba319863343c5fa32abf94d034f0c8dbfd223776): more failing tests with a fix

## [nest-0.3.1](https://github.com/ahdinosaur/nest/tree/nest-0.3.1)

- [4270449](https://github.com/ahdinosaur/nest/commit/4270449c6c2ad64e8875051496ec8a992d4ae335): add failing test case, and fix!
- [9179467](https://github.com/ahdinosaur/nest/commit/91794675f63868697365558b51b5c276ce7974fc): more friendly example

## [nest-0.3.0](https://github.com/ahdinosaur/nest/tree/nest-0.3.0)

- [bd4fc76](https://github.com/ahdinosaur/nest/commit/bd4fc768f4d4d903dc84c95b946326a15288f094): fix doc tests to pass
- [f15ab62](https://github.com/ahdinosaur/nest/commit/f15ab62f25719de5b048341235c923c5207c324d): no unwraps in example
- [dbe1691](https://github.com/ahdinosaur/nest/commit/dbe169174f55358d63d9792193db4833240d135c): first pass of module documentation!
- [f4c535c](https://github.com/ahdinosaur/nest/commit/f4c535c1c74bd5c31475580f734eb605748a13c3): change `Store#sub` to return Result not Option
- [1001d09](https://github.com/ahdinosaur/nest/commit/1001d098915914aa9416895a8c7b29e4cf6a1406): fix version numbers test

## [nest-0.2.0](https://github.com/ahdinosaur/nest/tree/nest-0.2.0)

- [c90a39e](https://github.com/ahdinosaur/nest/commit/c90a39e6c0448c3c1fe1a3535bea12e47b30dd35): add better example
- [485e5fd](https://github.com/ahdinosaur/nest/commit/485e5fd93502733dda8a47d2a926049d2d4eab94): move order of root and schema in Store::new
- [e2cb6df](https://github.com/ahdinosaur/nest/commit/e2cb6df096ac301b1d86d01fd44477508760a99f): rename walk to sub

## [nest-0.1.0](https://github.com/ahdinosaur/nest/tree/nest-0.1.0)

- [f10071f](https://github.com/ahdinosaur/nest/commit/f10071f8f2f9c8b0576a5a37c1eb983834cfcdc5): prepare for publishing
- [ec93538](https://github.com/ahdinosaur/nest/commit/ec9353863f69653617cf3c9631f04bdaca2ffcb4): better errors
- [8504daf](https://github.com/ahdinosaur/nest/commit/8504dafe30f9dac5ffe9cbd7275dad1a9a191d08): use path type for root
- [120f346](https://github.com/ahdinosaur/nest/commit/120f346296442a29778bd58a204d58aab5877169): str references all the way down
- [b25b5ec](https://github.com/ahdinosaur/nest/commit/b25b5ec5ae6053dfa9d9bc6c646a65d9d6b2df98): allow paths as String or str using
- [e553105](https://github.com/ahdinosaur/nest/commit/e55310588647cbf2e1c6e6e0927cad5f819ecd4f): better names
- [cbb7776](https://github.com/ahdinosaur/nest/commit/cbb7776cf615c7fa5800ff82c82ab7295758575e): create store from json
- [f72d8ec](https://github.com/ahdinosaur/nest/commit/f72d8ecf076e0187117de45612a4bd97ae133490): split out files
- [89aaba0](https://github.com/ahdinosaur/nest/commit/89aaba07c318d4182e4b3e9c0da6caea876e9818): split into lib and cli modules
- [ae58cd6](https://github.com/ahdinosaur/nest/commit/ae58cd65acf576f13c63d312fdcd411b650e14ee): implement set
- [148c28f](https://github.com/ahdinosaur/nest/commit/148c28f65556f8a5cb403ab462f976b9fb99c4d8): implement walk
- [264047e](https://github.com/ahdinosaur/nest/commit/264047e7707b9dd5e3586976509a2aee063573ef): Revert "hmm..."
- [4f2e62b](https://github.com/ahdinosaur/nest/commit/4f2e62ba1b57dc91e1fd0cba099dc72987258207): hmm...
- [3a10afc](https://github.com/ahdinosaur/nest/commit/3a10afc0c7ca327bfc2f80e12e8955ccdd4a68b5): hey it kinda works!
- [9e79b3b](https://github.com/ahdinosaur/nest/commit/9e79b3b45ef9cdb7956352635ebbf00df597cd87): in the beginning
