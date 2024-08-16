# web-skeleton-rs

Rust Web back-end skeleton

### start

- clone project

```sh
# clone without history
$ git clone --depth 1 <repository-url>
# if want refetch history
$ git fetch --unshallow
```

- start database

```sh
$ surreal start --user root --pass root --auth memory
```

- start server

```sh
APP_PORT=8080 APP_HOST=0.0.0.0 cargo watch -x 'run'
```

## Features

- [x] command line arg parse
- [x] configuration (config.rs)
- [ ] logger
- [x] router structure
- [x] module structure
- [x] middleware
- [ ] validator
- [ ] database connection pool
- [ ] ORM
- [ ] HTTP Client
- [x] Exception
- [ ] Response data define
- [ ] Open API Doc
- [x] AppState

## trace

- traceId:
- spanId:
- parentSpanId:
