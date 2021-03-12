## Building a Docker image

Raspberry Pi 2/3:

```sh
$ docker build -f Dockerfile.armv7-unknown-linux-gnueabihf --force-rm -t helium/cross:armv7-unknown-linux-gnueabihf-0.2.1 .
```

Raspberry Pi 4:

```sh
$ docker build -f Dockerfile.aarch64-unknown-linux-gnu --force-rm -t helium/cross:aarch64-unknown-linux-gnu-0.2.1 .
```
