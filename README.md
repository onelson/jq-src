# jq-src

This rust crate provides an api to compile `libjq` provided by the [jq]
**1.6** release.

The primary consumer of this crate is [jq-sys] which depends on this crate
when the `bundled` feature is enabled.

Setting `JQ_NO_STATIC` will build a shared lib instead of static (the default).
This is probably ill-advised since you'd then have to go out of your way to
ensure you retain the `libjq` build results and install them on your system
so they can be found at runtime. Still, the variable is there if you feel the
need to set it.

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
[jq-sys]: https://github.com/onelson/jq-sys