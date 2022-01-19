# Wordler

A Wordle solving assistant.

# What and Why?

Affected by the virally memetic game [wordle](https://www.powerlanguage.co.uk/wordle/), but linguistically
incapable of solving it, I set forth to develop a robotic assistant to do the hard work for me.

Wordler will suggest words to input into the game, then will listen for feedback about how it went, and use
it to suggest the next word. Repeat until you win (or lose, there's no guarantee).

# How?

Wordler is written in [rust](www.rust-lang.org) and has both a GUI and a CLI interface. Running 

    cargo build

will compile both. Try them, they should be self-explanatory enough.

# The Math?

It's weak. It's just using letter frequencies to determine a word score, and it suggests the highest-scoring
word compatible with the feedback received from the user.
