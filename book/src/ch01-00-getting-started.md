# Getting Started

At a high level, Komorei sources use network requests to read data from websites and parse it into
content that the Komorei application can read and display to users. A source can be written for any
website that provides content that can be grouped into "anime" (series) and "episodes", where each
episode resolves to one or more playable "streams" (video servers).

Komorei sources are [WebAssembly](https://webassembly.org/) programs, and the
[komorei-sdk](https://github.com/tachibana-shin/komorei-sdk) library and
[komorei cli](https://github.com/tachibana-shin/komorei-sdk/tree/main/crates/cli) tool enable you to
write Rust programs that can be compiled into source packages that the Komorei app runs.