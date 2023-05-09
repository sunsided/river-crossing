# Toy Planning: "Humans and Zombies"

An implementation of search-based planning using the Humans and Zombies problem,
a version of the river crossing problem without the racism of "[Missionaries and Cannibals]"
and the sexism of "Jealous Husbands".

The problem statement, paraphrasing Wikipedia, is this:

> In the humans and zombies problem, three humans and three zombies must cross
> a river using a boat which can carry at most two people, under the constraint that, for both banks,
> if there are humans present on the bank, they cannot be outnumbered by zombies
> (if they were, the zombies would eat the humans).
> The boat cannot cross the river by itself with no people on board.

Hit `cargo run` and observe a solution:

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

Result plans differ depending on whether a depth-first (LIFO) or
breadth-first (FIFO) search is used.

[Missionaries and Cannibals]: https://en.wikipedia.org/wiki/Missionaries_and_cannibals_problem
