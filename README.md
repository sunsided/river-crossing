# Toy Planning: River Crossing
_Nᴏᴡ ᴡɪᴛʜ x﹪ ᴍᴏʀᴇ ᴢᴏᴍʙɪᴇs ᴀɴᴅ ʙʀɪᴅɢᴇs﹗_

---

Implementation(s) of search-based planning on the [river crossing] type of ~~toy problems~~ puzzles.
Since this is really just a playground, the found solutions are not necessarily optimal,
i.e. shorter or more efficient paths may exist.

## Humans and Zombies

This is the Humans and Zombies problem, a classic version of the river crossing problem without
the racism of "[Missionaries and Cannibals]" and the sexism of "Jealous Husbands".

The problem statement, paraphrasing Wikipedia, is this:

> In the humans and zombies problem, three humans and three zombies must cross
> a river using a boat which can carry at most two people, under the constraint that, for both banks,
> if there are humans present on the bank, they cannot be outnumbered by zombies
> (if they were, the zombies would eat the humans).
> The boat cannot cross the river by itself with no people on board.

See [`humans_and_zombies.rs`](src/humans_and_zombies.rs) for the problem specifics
and [`search.rs`](src/search.rs) for the search specifics or simply run `cargo run -- humans-and-zombies` and observe a solution:

```
  HHH ZZZ |B~~~|
           ZZ →
    HHH Z |~~~B| ZZ
           ← Z
   HHH ZZ |B~~~| Z
           ZZ →
      HHH |~~~B| ZZZ
           ← Z
    HHH Z |B~~~| ZZ
           HH →
      H Z |~~~B| HH ZZ
           ← H Z
    HH ZZ |B~~~| H Z
           HH →
       ZZ |~~~B| HHH Z
           ← Z
      ZZZ |B~~~| HHH
           ZZ →
        Z |~~~B| HHH ZZ
           ← H
      H Z |B~~~| HH ZZ
           H Z →
          |~~~B| HHH ZZZ
```

You can parameterize the problem. To use only two zombies and a boat with capacity four, run e.g.

```
cargo run -- humans-and-zombies --humans 3 --zombies 2 --boat 4
```

Which prints a plan like:

```
   HHH ZZ |B~~~|
           HHH →
       ZZ |~~~B| HHH
           ← HH
    HH ZZ |B~~~| H
           HH ZZ →
          |~~~B| HHH ZZ
```

Result plans differ depending on whether a depth-first (LIFO) or
breadth-first (FIFO) search is used.

## Bridge and Torch

The [Bridge and Torch] problem works as follows:

> Four people come to a river in the night. There is a narrow bridge, but it can only
> hold two people at a time. They have one torch and, because it's night, the torch has
> to be used when crossing the bridge.
> 
> Person A can cross the bridge in 1 minute, B in 2 minutes, C in 5 minutes, and D in 8 minutes.
> When two people cross the bridge together, they must move at the slower person's pace.
> 
> The question is, can they all get across the bridge if the torch lasts only 15 minutes?

To solve the problem, run:

```
cargo run -- bridge-and-torch
```

It prints a solution like the following:

```
  At 0 minutes: [<1>, <2>, <5>, <8>] on the left, nobody on the right (torch: 15 minutes)
   → [<1>, <2>] cross forward, taking 2 minutes
  At 2 minutes: [<5>, <8>] on the left, [<1>, <2>] on the right (torch: 13 minutes)
   ← [<1>] returns, taking 1 minute
  At 3 minutes: [<5>, <8>, <1>] on the left, [<2>] on the right (torch: 12 minutes)
   → [<5>, <8>] cross forward, taking 8 minutes
  At 11 minutes: [<1>] on the left, [<2>, <5>, <8>] on the right (torch: 4 minutes)
   ← [<2>] returns, taking 2 minutes
  At 13 minutes: [<1>, <2>] on the left, [<5>, <8>] on the right (torch: 2 minutes)
   → [<1>, <2>] cross forward, taking 2 minutes
  At 15 minutes: nobody on the left, [<5>, <8>, <1>, <2>] on the right (torch: 0 minutes)
```

[River crossing]: https://en.wikipedia.org/wiki/River_crossing_puzzle
[Missionaries and Cannibals]: https://en.wikipedia.org/wiki/Missionaries_and_cannibals_problem
[Bridge and Torch]: https://en.wikipedia.org/wiki/Bridge_and_torch_problem
