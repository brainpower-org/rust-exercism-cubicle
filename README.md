# rust-exercism-cubicle 

Work on the Rust track of exercism in a turn-key environment.

![](https://img.shields.io/circleci/project/github/marionebl/rust-exercism-cubicle/master.svg?style=flat-square)
![](https://img.shields.io/docker/cloud/build/marionebl/rust-exercism-cubicle.svg?label=docker&style=flat-square)

## Prerequesites

- docker
- docker-compose

## Getting started

```sh
docker-compose pull
docker-compose up
```

## Run tests for one package

```
cargo test -p 'hello-world'
```

## Watch tests for one package

```
cargo watch -x 'test -p hello-world'
```