# t-ranking

# Setup

1. Set`ASPEN_USERNAME` and `ASPEN_PASSWORD` environment variables. This data will eventually be obtained by the front end, but in development it does not matter.
2. run `cargo run` to launch the backend on **_127.0.0.1:8000_**. Compilation may take a while the first time as some heavy libraries need to be compiled. On subsequent runs, compilation should be much faster.

# Cool Tree

```
src - where rust source code is stored
└─| main.rs - entry point for rocket backend
  | lib.rs - module definitions
  └─| aspen.rs - functions for interacting with aspen such as getting response and classes
hooks - where all git hooks would be
└─| pre-commit - the shell script to be ran before commits
  | post-commit - the shell script to be ran after commits

```

TODOS:

-   [] Start vue.js frontend
-   [] Set up postgresql for storing ranking data
-   [] Fix issue in hooks/pre-commit where rustfmt does not recognize formatting options
