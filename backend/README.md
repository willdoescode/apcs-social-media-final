# backend

#### Dependencies

```
rust

docker

postgresql
```

#### Run instructions

```bash
$ docker compose up

$ cargo install diesel_cli
$ diesel migration run

$ cargo build --release
$ ./target/release/backend
```
