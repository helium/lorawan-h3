# LoRaWAN H3

LoRaWAN regions represented as serialized (little endian) [H3] indices which
build a h3 map based on a geojson source file.

`lw-generator` is used to generate a compressed h3 region index file (extension
`.h3idz`) for a given region [upstream geojson]. Note that generating h3 region
index files can take a VERY long time.

The h3 region index files are built as [release assets] for this repository when
tagged with a `region_maps_YYYY.MM.DD` tag

`lw-generator` is also used to generate compressed region parameters for a given
[region parameters json] file.

The region parameter files are built as [release assets] for this repository
when tagged with a `region_params_YYYY.MM.DD` tag

Supported regions are listed in the [regions.txt] file. New supported regions
_must_ be added there with corresponding [upstream geojson] and [region
parameters json] files.

#### Example index

Generate IN865.res7.h3irz fom IN865.geojson.

```
$ target/release/lw-generator regions generate extern/hplans/IN865.geojson IN865.res7.h3idz
```

A `Makefile` is supplied to make manual generation easier, but note that index
or region parameter files are pushed as [release assets] by CI infrastructure when
the main branch is tagged.

```
$ make compile
$ make -j index
```

#### Example region parameters

Generate IN865.rpz from IN865.json

```
$ target/release/lw-generator params generate region_params/IN865.json IN865.rpz
```

The suplied `Makefile` can make manual generation easier, but note that index
or region parameter files are pushed as release assets by CI infrastructure when
the main branch is tagged.

```
$ make compile
$ make -j params
```

### lw-generator

`lw-generator` is the tool used to generate or export region index maps or region parameter files

#### Build

Native build:

```
$ cargo build --release
```

or using the `Makefile`:

```
$ make compile
```

#### Release

Release a new version of `lw-generator` using [cargo release] with one of the supported release options.

A release will be built by CI and pushed as a [release assets]. To use the
release in region index or region parameter generation, make sure to adjust the
version in the `Setup | Tools` section for in the [region_maps.yml] and
[region_params.yml] CI files.

<!-- Links -->

[h3]: https://h3geo.org
[upstream geojson]: https://github.com/dewi-alliance/hplans
[region parameters json]: https://github.com/helium/lorawan-h3/tree/main/region_params
[cargo release]: https://crates.io/crates/cargo-release
[release assets]: https://github.com/helium/lorawan-h3/releases
[regions.txt]: https://github.com/helium/lorawan-h3/blob/main/regions.txt
[region_params.yml]: https://github.com/helium/lorawan-h3/blob/main/.github/workflows/region_params.yml
[region_maps.yml]: https://github.com/helium/lorawan-h3/blob/main/.github/workflows/region_maps.yml
