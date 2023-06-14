# SCU (Shortcut Command Unifier)

Replacement for [Binclude](https://github.com/albertoaer/binclude)

Able to create command shortcuts to `commands`/`scripts`/`utilities` and make them available in multiple formats for multiple interpreters.

## How does it works?

- The directory `scu_data` is setted up if it is not yet. It contains:
  - `meta` where shortcut template metadata are stored
  - `bin` where templates have been converted into executable scripts