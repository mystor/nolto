# nolto

So, you want to not lto your rust code in gecko?

```
cargo build --release # The results are in /target/release
```

then do one of the following:

## I use `sccache`

Replace your `--enable-ccache=sccache` line with:

```
--enable-ccache=/path/to/target/release/nolto-sccache
```

## I don't use `sccache`

Add

```
mk_add_options "export RUSTC_WRAPPER=/path/to/target/release/nolto"
```

## Why can't I use the RUSTC_WRAPPER line with `sccache`

'cause we clobber RUSTC_WRAPPER when using sccache in our configuration.
