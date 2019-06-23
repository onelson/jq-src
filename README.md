# jq-src

[![crates.io](https://img.shields.io/crates/v/jq-src.svg)](https://crates.io/crates/jq-src)
[![crates.io](https://img.shields.io/crates/d/jq-src.svg)](https://crates.io/crates/jq-src)
[![docs.rs](https://docs.rs/jq-src/badge.svg)](https://docs.rs/jq-src)

This rust crate provides an api to compile `libjq` provided by the [jq]
**1.6** release. Other versions of jq _may work_ but haven't been tested.

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

## Changelog

### v0.4.1 (2019-06-22)

- Fixes issue where dependent crates can fail to build while linking on some systems ([#3]).

### v0.4.0 (2019-06-09)

- Upgrade to `autotools` 0.2, updating the usages to be compatible with the
  new public api.
- (Hopefully) Fixes issue where the autotools build can intermittently fail
  ([#1]).

### v0.3.1 (2019-06-01)

- Downgrade to `autotools` 0.1 (newer versions cause breakage in jq build).

### v0.3.0 (2019-06-01)

- Removed `Artifacts::print_cargo_metadata()`, which was largely duplicate of
  `Artifacts::print_link_info()` (somehow I wrote this method twice).
- Added some docs.

### v0.2.0 (2019-02-18)

- No longer build/link in `build.rs` - add wrapper API so [jq-sys] can do it
  itself.
- Try improve build script reliability by cleaning jq src dir prior to each
  run (if it exists).

### v0.1.0 (2019-01-12)

Initial release.


[jq]: https://github.com/stedolan/jq
[crates.io]: https://crates.io/
[oniguruma]: https://github.com/kkos/oniguruma/
[jq-sys]: https://github.com/onelson/jq-sys
[#1]: https://github.com/onelson/jq-src/issues/1
[#3]: https://github.com/onelson/jq-src/issues/3
