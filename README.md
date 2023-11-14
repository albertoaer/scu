# SCU (Shortcut Command Unifier)

Replacement for [Binclude](https://github.com/albertoaer/binclude)

Small utility able to create command shortcuts to `commands`/`scripts`/`utilities` and make them available in multiple formats for multiple interpreters.

## How does it works?

The purpose of **scu** is to produce a shortcut to a command or script that can be launched from several interpreters, like many programming languages already do so their binaries could be invoked from multiple terminals. For example: creating a *.cmd*, a *.sh* (in this case better with no extension), a *.ps1* and even a *.py*.

**scu** can create templates for the shortcuts, and apply them later with the desired set of interpreters.

- Everything is stored in the directory `scu_data`, it contains:
  - `meta`: the templates
  - `bin`: generated scripts (the shortcuts)
- In order to work you must ensure `bin` is included in the system *PATH*

`bin` location can be obtained using:
```sh
$ scu bin
```