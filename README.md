LoRaWAN H3 Regions
==================

LoRaWAN regions represented as serialized (little endian) H3 indices.

## Building

This can take an EXTREMELY long time, which is why the build by
products are included in this repository.

```
$ make compile
$ make
$ make -j
```

Two calls to make are required due to makefile dependency ordering
(PRs welcome).
