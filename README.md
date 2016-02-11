# multipart bug - First entry is ignored

This is an executable example.

To start the server:
```
cargo run
```

This will start a server listening on `0.0.0.0:7000`.

Reach out on a browser for http://localhost:7000 and any 3 files.

The expected result is to have the 3 field names printed on the console.
```
Running on 0.0.0.0:7000
Entry name: "banana"
Entry name: "lime"
Entry name: "pear"
```

The actual result skips the first field:
```
Running on 0.0.0.0:7000
Entry name: "lime"
Entry name: "pear"
```

## Dependencies versions

```
hyper     0.7.2
multipart 0.4.0
cargo 0.7.0-nightly (1af03be 2015-12-08)
rustc 1.6.0 (c30b771ad 2016-01-19)
```
