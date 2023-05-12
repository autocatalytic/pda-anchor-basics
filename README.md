## Solana and Anchor Basics

Personal work learning the Anchor framework. Solana's massively distributed database remains almost total magic to me but I'm determined to get a technical handle on it. Anchor should help in this effort but learning both at the same time has been harder than I expected, primarily because the separation of data/state and message passing instructions creates a huge intuition blocker for a crusty RDBMS dude.

Motivation: better understand Solana PDA's and Anchor
Tools: Solana CLI, @coral-xyz/anchor, vscode
Recipe: SolAndy's video at https://www.youtube.com/watch?v=VWZAXXygVOM

## Awesome Outcomes

Getting a basic system up and running now happens quickly and brings me to creative mode much more quickly. For example, #[derive(Accounts)] is not required for basic access to running Solana rust commands, you just need these blocks when accessing interesting things like account data. Kind of like running a front end and app server, but you only need the DB once in a while.


Uniqueness constraints when using static PDA seeds slowed me down a bunch, as is switching between system-program, PDA's and signature authorities. Nothing on the latter in this work, but I know it's still a weakness.

Getting to know someone through their videos is something else! Doing that quite a bit during and after the pandemic, and certainly the case here as well. Future of education? LOL.
