# LoRaWAN H3

LoRaWAN regions represented as serialized (little endian) [H3] indices.

`lw-generator` is used to generate compressed h3 index file for a given region
geojson file. Note that generating h3 index files can take a VERY long time.

The h3 index files are built as release assets for this repository when tagged.

#### Example

Generate IN865.res7.h3irz fom IN865.geojson.

```
$ target/release/lw-generator generate extern/hplans/IN865.geojson IN865.res7.h3idz
```

A `Makefile` is supplied to make manual generation easier, but note that index
files are pushed as release assets by CI infrastructure when the main branch is
tagged.

```
$ make compile
$ make -j
```

#### Build

Native build:

```
$ cargo build --release
```

or using the `Makefile`:

```
$ make compile
```

<!-- Links -->

[h3]: https://h3geo.org
[upstream geojson]: https://github.com/dewi-alliance/hplans
