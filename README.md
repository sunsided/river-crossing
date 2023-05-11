# Toy Planning: River Crossing
_Nᴏᴡ ᴡɪᴛʜ x﹪ ᴍᴏʀᴇ ᴢᴏᴍʙɪᴇs ᴀɴᴅ ʙʀɪᴅɢᴇs﹗_

---

Implementation(s) of search-based planning on the [river crossing] type of ~~toy problems~~ puzzles.
Since this is really just a playground, the found solutions are not necessarily optimal,
i.e. shorter or more efficient paths may exist. The core state-space search logic is implemented in
[`search.rs`](src/search.rs).

## 🌉+🔦 — The Bridge and Torch Problem

The [Bridge and Torch] problem works as follows:

> Four people come to a river in the night. There is a narrow bridge, but it can only
> hold two people at a time. They have one torch and, because it's night, the torch has
> to be used when crossing the bridge.
>
> Person A can cross the bridge in 1 minute, B in 2 minutes, C in 5 minutes, and D in 8 minutes.
> When two people cross the bridge together, they must move at the slower person's pace.
>
> The question is, can they all get across the bridge if the torch lasts only 15 minutes?

The problem specifics are encoded in [`bridge_and_torch.rs`](src/problems/bridge_and_torch.rs).
To solve the problem, run either of these equivalent commands;

```
cargo run -- bridge-and-torch
cargo run -- bridge-and-torch --bridge 2 --torch 15 --person 1 --person 2 --person 5 --person 8
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

## 🐺+🐐+🥬 — The Wolf, Goat and Cabbage Problem

The [Wolf, Goat and Cabbage] problem works as follows:

> A farmer went to a market and purchased a wolf, a goat, and a cabbage. 
> On his way home, the farmer came to the bank of a river and rented a boat.
> But crossing the river by boat, the farmer could carry only himself and a
> single one of his purchases: the wolf, the goat, or the cabbage.
>
> If left unattended together, the wolf would eat the goat, or the goat would eat the cabbage.
> 
> The farmer's challenge was to carry himself and his purchases to the far
> bank of the river, leaving each purchase intact.

The problem specifics are encoded in [`wolf_goat_cabbage.rs`](src/problems/wolf_goat_cabbage.rs).
To solve the problem, run either of these equivalent commands;

```
cargo run -- wolf-goat-cabbage
cargo run -- wolf-goat-cabbage --boat 2 --farmers 1 --wolves 1 --goats 1 --cabbage 1
```

It prints a solution like the following:

```
  At t=0; left bank: farmer, wolf, goat and cabbage; right bank: empty
   → farmer and goat cross forward
  At t=1; left bank: wolf and cabbage; right bank: farmer and goat
   ← farmer returns alone
  At t=2; left bank: farmer, wolf and cabbage; right bank: goat
   → farmer and cabbage cross forward
  At t=3; left bank: wolf; right bank: farmer, goat and cabbage
   ← farmer and goat return
  At t=4; left bank: farmer, wolf and goat; right bank: cabbage
   → farmer and wolf cross forward
  At t=5; left bank: goat; right bank: farmer, wolf and cabbage
   ← farmer returns alone
  At t=6; left bank: farmer and goat; right bank: wolf and cabbage
   → farmer and goat cross forward
  At t=7; left bank: empty; right bank: farmer, wolf, goat and cabbage
```

## 🙎+🧟‍ — The Humans and Zombies Problem

This is the Humans and Zombies problem, a classic version of the river crossing problem without
the racism of "[Missionaries and Cannibals]" and the sexism of "Jealous Husbands".

The problem statement, paraphrasing Wikipedia, is this:

> In the humans and zombies problem, three humans and three zombies must cross
> a river using a boat which can carry at most two people, under the constraint that, for both banks,
> if there are humans present on the bank, they cannot be outnumbered by zombies
> (if they were, the zombies would eat the humans).
> The boat cannot cross the river by itself with no people on board.

See [`humans_and_zombies.rs`](src/problems/humans_and_zombies.rs) for the problem specifics or
simply run `cargo run -- humans-and-zombies` and observe a solution:

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

[River crossing]: https://en.wikipedia.org/wiki/River_crossing_puzzle
[Missionaries and Cannibals]: https://en.wikipedia.org/wiki/Missionaries_and_cannibals_problem
[Bridge and Torch]: https://en.wikipedia.org/wiki/Bridge_and_torch_problem
[Wolf, Goat and Cabbage]: https://en.wikipedia.org/wiki/Wolf,_goat_and_cabbage_problem
