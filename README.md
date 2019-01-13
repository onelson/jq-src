# jq-src

This rust crate will compile and statically link `libjq` provided by the [jq]
**1.6** release.
As per the [jq] readme, the library is compiled using the built-in [oniguruma]
library for regex support.

For this to succeed, you will have to have autotools (`autoreconf`, `make`, 
etc) and a gcc toolchain in your `PATH`.

Note that the jq sources are provided as git submodules. These will be fetched
automatically when installing this crate from [crates.io] or using a git dependency,
however if you want to hack on the crate locally, or list it as a path dependency, 
you'll have to fetch the submodules yourself, for example:

```
$ git submodule update --init --recursive
```

If building this crate fails because of missing files, it's likely the submodules
were not initialized.

[jq]: https://github.com/stedolan/jq
[crates.io]: https://crates.io/
[oniguruma]: https://github.com/kkos/oniguruma/