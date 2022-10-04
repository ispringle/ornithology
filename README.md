# Ornithology

## Introduction

Ornithology is a Rust library which exposes a number of combinators, in the form of bird names.
At the moment, this library should be seen as a port of Haskell's
[Data.Aviary.Birds](https://hackage.haskell.org/package/data-aviary-0.4.0/docs/Data-Aviary-Birds.html)
package. I am also taking some inspiration from the JavaScript library
[fantasy-birds](https://github.com/fantasyland/fantasy-birds).

While Data.Aviary.Birds is largely an example package and has no intended us outside of
education, I intended that Ornithology will be a usable library in an actual software, if the
need or desire for combinators arises.

Ornithology requires using nightly build due to some needed `features`.

## Birds

- [ ] becard: `(c -> d) -> (b -> c) -> (a -> b) -> a -> d`
- [ ] blackbird: `(c -> d) -> (a -> b -> c) -> a -> b -> d`
- [ ] bluebird': `(a -> c -> d) -> a -> (b -> c) -> b -> d`
- [ ] bunting: `(d -> e) -> (a -> b -> c -> d) -> a -> b -> c -> e`
- [ ] cardinal: `(a -> b -> c) -> b -> a -> c`
- [ ] cardinal': `(c -> a -> d) -> (b -> c) -> a -> b -> d`
- [ ] cardinalstar: `(a -> c -> b -> d) -> a -> b -> c -> d`
- [ ] cardinalstarstar: `(a -> b -> d -> c -> e) -> a -> b -> c -> d -> e`
- [ ] dove: `(a -> c -> d) -> a -> (b -> c) -> b -> d`
- [ ] dickcissel: `(a -> b -> d -> e) -> a -> b -> (c -> d) -> c -> e`
- [ ] dovekie: `(c -> d -> e) -> (a -> c) -> a -> (b -> d) -> b -> e`
- [ ] eagle: `(a -> d -> e) -> a -> (b -> c -> d) -> b -> c -> e`
- [ ] eaglebald: `(e -> f -> g) -> (a -> b -> e) -> a -> b -> (c -> d -> f) -> c -> d -> g`
- [ ] finch: `a -> b -> (b -> a -> c) -> c`
- [ ] finchstar: `(c -> b -> a -> d) -> a -> b -> c -> d`
- [ ] finchstarstar: `(a -> d -> c -> b -> e) -> a -> b -> c -> d -> e`
- [ ] goldfinch: `(b -> c -> d) -> (a -> c) -> a -> b -> d`
- [ ] hummingbird: `(a -> b -> a -> c) -> a -> b -> c`
- [ ] idiot: `a -> a`
- [ ] idstar: `(a -> b) -> a -> b`
- [ ] idstarstar: `(a -> b -> c) -> a -> b -> c`
- [ ] jalt: `(a -> c) -> a -> b -> c`
- [ ] jalt': `(a -> b -> d) -> a -> b -> c -> d`
- [ ] jay: `(a -> b -> b) -> a -> b -> a -> b`
- [ ] kestrel: `a -> b -> a`
- [ ] kite: `a -> b -> b`
- [ ] owl: `((a -> b) -> a) -> (a -> b) -> b`
- [ ] phoenix: `(b -> c -> d) -> (a -> b) -> (a -> c) -> a -> d`
- [ ] psi: `(b -> b -> c) -> (a -> b) -> a -> a -> c`
- [ ] quacky: `a -> (a -> b) -> (b -> c) -> c`
- [ ] queer: `(a -> b) -> (b -> c) -> a -> c`
- [ ] quirky: `(a -> b) -> a -> (b -> c) -> c`
- [ ] quixotic: `(b -> c) -> a -> (a -> b) -> c`
- [ ] quizzical: `a -> (b -> c) -> (a -> b) -> c`
- [ ] robin: `a -> (b -> a -> c) -> b -> c`
- [ ] robinstar: `(b -> c -> a -> d) -> a -> b -> c -> d`
- [ ] robinstarstar: `(a -> c -> d -> b -> e) -> a -> b -> c -> d -> e`
- [ ] starling: `(a -> b -> c) -> (a -> b) -> a -> c`
- [ ] starling': `(b -> c -> d) -> (a -> b) -> (a -> c) -> a -> d`
- [ ] thrush: `a -> (a -> b) -> b`
- [ ] vireo: `a -> b -> (a -> b -> c) -> c`
- [ ] vireostar: `(b -> a -> b -> d) -> a -> b -> b -> d`
- [ ] vireostarstar: `(a -> c -> b -> c -> e) -> a -> b -> c -> c -> e`
- [ ] warbler: `(a -> a -> b) -> a -> b`
- [ ] warbler1: `a -> (a -> a -> b) -> b`
- [ ] warblerstar: `(a -> b -> b -> c) -> a -> b -> c`
- [ ] warblerstarstar: `(a -> b -> c -> c -> d) -> a -> b -> c -> d`
