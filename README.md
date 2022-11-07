# `simple-sat`

One of the SAT solvers of all time 🤓

## Usage

`simple-sat` accepts SAT formulae in [DIMACS format](http://www.satcompetition.org/2009/format-benchmarks2009.html).
To invoke the solver, run:
```shell
simple-sat -i <INPUT-FILE> -o <OUTPUT-FILE>
```
which will run solver on `<INPUT-FILE>` and write into `<OUTPUT-FILE>` one of:
- `UNSAT` if the formula is unsatisfiable,
- `SAT` and the satisfying assignment otherwise, separated by `\n`.

The solver produces logs of the search process; to configure their verbosity, set up `RUST_LOG` environment variable.
Available levels are, in decreasing order of verbosity, `trace`, `debug`, `info`, `warn`, `error` and `off`
(defaults to `info`).

Additionally, `simple-sat` allows to choose the solver engine (CDCL and brute-force) and branching heuristic
(first unassigned and VSIDS). See `simple-sat --help` for more details.

## Build

This is a Cargo project, so it follows the standard convention of the Cargo build system. For example,
to make the production build, run `cargo build --release` from this directory. 