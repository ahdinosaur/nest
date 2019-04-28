# Changelog

with help from [`git log`](https://www.git-scm.com/docs/git-log):

```shell
git log --oneline --format="- [%h](https://github.com/ahdinosaur/nest/commit/%H): %s"
```

## Unreleased

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
