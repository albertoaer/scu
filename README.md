# SCU (Shortcut Command Unifier)

Replacement for [Binclude](https://github.com/albertoaer/binclude)

Small utility able to create command shortcuts to `commands`/`scripts`/`utilities` and make them available in multiple formats for multiple interpreters.

## How does it works?

- The directory `scu_data` is setted up if it is not yet. It contains:
  - `meta` where shortcut template metadata are stored
  - `bin` where templates have been converted into executable scripts
- `scu` manipulates the templates located at `meta` and produces the executable scripts into `bin`
- In order to work you must ensure `bin` is included in the system *PATH*

`bin` location can be obtained using:
```sh
$ scu binaries
```