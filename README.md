# Exit With Code (EWC) - CLI

## Overview

```text
ewc 1.0.0
Martijn Gribnau <garm@ilumeo.com>

Returns the exit code, provided by you as the first argument, unless:
* Too many or too few arguments were given
* The argument could not be parsed as UTF-8
* The argument could not be parsed as an integer (i32)
* Platform specific behaviour, such as the maximum accepted range of integers, interfered with the
  execution behaviour of the program

When the program fails with an unexpected error, as above, it will output a message to stderr,
and exit with exit code 255. The program will not write to stderr in case of successful execution
of the program. NB: A non-zero exit code on its own does not indicate failure for this program.

Usage:
ewc [EXIT_CODE]

Args:
EXIT_CODE   The exit code which this program will return
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.