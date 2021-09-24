LoRaWAN H3
==========

LoRaWAN regions represented as serialized (little endian) [H3] indices.

## Project Layout

### `cli/`

<details>

<summary>
Source code for the `lwr` CLI tool.
</summary>

`lwr` is a self-contained static binary which returns the
LoRaWAN region, if any, for any geographic location on
earth.

#### Example

Look up latitude, longitude [1, 0]:

```
$ ./target/release/lwr 1 0
searching US915 for 8c754331db055ff
searching EU868 for 8c754331db055ff
searching AS923_1 for 8c754331db055ff
searching AS923_2 for 8c754331db055ff
searching AS923_3 for 8c754331db055ff
searching AU915 for 8c754331db055ff
searching CN470 for 8c754331db055ff
searching EU433 for 8c754331db055ff
searching IN865 for 8c754331db055ff
searching KR920 for 8c754331db055ff
searching RU864 for 8c754331db055ff
```

#### Build

Native build:

```
$ cargo build --release
```

With [`cross`]:

First build an appropriate docker image according to the instructions
in the `docker/` section below. For this example we'll be bulding for
Raspberry Pi 4.

```
$ cross build --release --target aarch64-unknown-linux-gnu
```
</details>

### `docker/`

<details>

<summary>
Dockerfiles needed to compile `lwr` with [`cross`].
</summary>

The Dockerfiles in this directory are only needed when cross-compiling
`lwr` CLI with `cross`. `cross` normally installs the correct docker
image for you as necessary, but its images do not have `libclang`
which is needed by `lwr`'s `h3ron-h3-sys` dependency.

If you do not need to build `lwr` with `cross` then you can ignore
this directory.

## Building the images

Raspberry Pi 2/3:

```sh
$ docker build -f Dockerfile.armv7-unknown-linux-gnueabihf --force-rm -t helium/cross:armv7-unknown-linux-gnueabihf-0.2.1 .
```

Raspberry Pi 4:

```sh
$ docker build -f Dockerfile.aarch64-unknown-linux-gnu --force-rm -t helium/cross:aarch64-unknown-linux-gnu-0.2.1 .
```

</details>

### `generator/`

<details>

<summary>
Source code for polyfill generator library.
</summary>

#### Build

```
$ rebar3 compile
```
</details>

### `seraizlized/`
<details>

<summary>
Pre-generated serialized H3 regions.
</summary>

#### Build

This can take an EXTREMELY long time, which is why the build by
products are included in this repository. Regenerating these files are
only needed if the [upstram geojson] changes.


First build the erlang code as documented in the `generator/` section above.

```
$ make compile
$ make
$ make -j
```

Multiple calls to make are required due to makefile dependency
ordering (PRs welcome).

</details>


<!-- Links -->

[`cross`]: https://github.com/rust-embedded/cross
[docker/README.md]: docker/README.md
[1, 0]: https://goo.gl/maps/34w9pcQfzG2NJfVV8
[H3]: https://h3geo.org
[upstram geojson]: https://github.com/gradoj/hplans
