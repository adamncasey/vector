---
title: Install Vector From Source
sidebar_label: From Source
description: Install Vector from the Vector source code
---

This page covers installing Vector from source. Because Vector is written in
[Rust][urls.rust] it can compile to a single static binary. You can view an
example of this in the [musl builder Docker image][urls.musl_builder_docker_image].

import Alert from '@site/src/components/Alert';

<Alert type="warning">

We recommend installing Vector through a supported [container
platform][docs.containers], [package manager][docs.package_managers], or 
pre-built [archive][docs.from_archives], if possible. These handle permissions,
directory creation, and other intricacies covered in the [Next
Steps](#next-steps) section.

</Alert>

## Installation

import Tabs from '@theme/Tabs';

1.  Install Rust

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    ```

2. Install C++ toolchain

    Install C and C++ compilers (GCC or Clang) and GNU `make` if they are not pre-installed
    on your system.

3.  Create the `vector` directory

    ```bash
    mkdir vector
    ```

4.  Download Vector's Source
  
    <Tabs
      className="mini"
      defaultValue="latest"
      values={[
        { label: 'Latest (0.5.0)', value: 'latest'},
        { label: 'Master', value: 'master'},
      ]}>

    import TabItem from '@theme/TabItem';

    <TabItem value="latest">

    ```bash
    mkdir -p vector && \
      curl -sSfL --proto '=https' --tlsv1.2 https://github.com/timberio/vector/archive/v0.5.0.tar.gz | \
      tar xzf - -C vector --strip-components=1
    ```

    </TabItem>
    <TabItem value="master">

    ```bash
    mkdir -p vector && \
      curl -sSfL --proto '=https' --tlsv1.2 https://github.com/timberio/vector/archive/master.tar.gz | \
      tar xzf - -C vector --strip-components=1
    ```

    </TabItem>
    </Tabs>

5.  Change into the `vector` directory

    ```bash
    cd vector
    ```

6.  Compile Vector

    ```bash
    [FEATURES="<flag1>,<flag2>,..."] make build
    ```

    The `FEATURES` environment variable is optional. You can override the
    default features with this variable. See [feature flags](#feature-flags)
    for more info.

    When finished, the vector binary will be placed in `target/<target>/release/vector`.
    For example, if you are building Vector on your Mac, your target triple
    is `x86_64-apple-darwin`, and the Vector binary will be located at
    `target/x86_64-apple-darwin/release/vector`.

7.  Start Vector

    Finally, start vector:

    ```bash
    target/<target>/release/vector --config config/vector.toml
    ```

## Next Steps

### Configuring

The Vector configuration file is located at:

```
config/vector.toml
```

A full spec is located at `config/vector.spec.toml` and examples are
located in `config/vector/examples/*`. You can learn more about configuring
Vector in the [Configuration][docs.configuration] section.

### Data Directory

We highly recommend creating a [data directory][docs.global-options#data-directory]
that Vector can use:

```
mkdir /var/lib/vector
```

<Alert type="warning">

Make sure that this directory is writable by the `vector` process.

</Alert>

Vector offers a global [`data_dir` option][docs.global-options#data_dir] that
you can use to specify the path of your directory.

import CodeHeader from '@site/src/components/CodeHeader';

<CodeHeader fileName="vector.toml" />

```toml
data_dir = "/var/lib/vector" # default
```

### Service Managers

Vector archives ship with service files in case you need them:

#### Init.d

To install Vector into Init.d run:

```bash
cp -av etc/init.d/vector /etc/init.d
```

#### Systemd

To install Vector into Systemd run:

```bash
cp -av etc/systemd/vector /etc/systemd/system
```

### Updating

Simply follow the same [installation instructions above](#installation).

## How It Works

### Feature Flags

The following feature flags are supported via the `FEATURES` env var when
executing `make build`:

```bash
[FEATURES="<flag1>,<flag2>,..."] make build
```

| Feature | Description | Enabled by default |
| :------ | :---------- | :----------------- |
| `jemallocator` | Enables vendored [jemalloc][urls.jemalloc] instead of default memory allocator, which improves [performance][docs.performance]. | <i className="feather icon-check"></i> |
| `leveldb` | Enables support for [disk buffers][docs.glossary#buffer] using vendored [LevelDB][urls.leveldb]. | <i className="feather icon-check"></i> |
| `leveldb/leveldb-sys-2` | Can be used together with `leveldb` feature to use LevelDB from [`leveldb-sys` 2.x][urls.leveldb-sys-2] crate, which doesn't require `cmake` as build dependency, but supports less platforms. | <i className="feather icon-check"></i> |
| `leveldb/leveldb-sys-3` | Can be used together with `leveldb` feature to use LevelDB from development version of [`leveldb-sys` 3.x][urls.leveldb-sys-3] crate, which requires `cmake` as build dependency, but supports more platforms. | |
| `openssl/vendored` | Enables vendored [OpenSSL][urls.openssl]. If disabled, system SSL library is used instead. | <i className="feather icon-check"></i> |
| `rdkafka` | Enables vendored [librdkafka][urls.lib_rdkafka] dependency, which is required for [`kafka` source][docs.sources.kafka] and [`kafka` sink][docs.sources.kafka]. | <i className="feather icon-check"></i> |
| `rdkafka/cmake_build` | Can be used together with `rdkafka` feature to build `librdkafka` using `cmake` instead of default build script in case of build problems on non-standard system configurations. | |
| `shiplift/unix` | Enables support for Unix domain sockets in [`docker`][docs.sources.docker] source. | <i className="feather icon-check"></i> |


[docs.configuration]: /docs/setup/configuration
[docs.containers]: /docs/setup/installation/containers
[docs.from_archives]: /docs/setup/installation/manual/from-archives
[docs.global-options#data-directory]: /docs/reference/global-options#data-directory
[docs.global-options#data_dir]: /docs/reference/global-options#data_dir
[docs.glossary#buffer]: /docs/meta/glossary#buffer
[docs.package_managers]: /docs/setup/installation/package-managers
[docs.performance]: /docs/about/performance
[docs.sources.docker]: /docs/reference/sources/docker
[docs.sources.kafka]: /docs/reference/sources/kafka
[urls.jemalloc]: https://github.com/jemalloc/jemalloc
[urls.leveldb-sys-2]: https://crates.io/crates/leveldb-sys
[urls.leveldb-sys-3]: https://github.com/timberio/leveldb-sys/tree/v3.0.0
[urls.leveldb]: https://github.com/google/leveldb
[urls.lib_rdkafka]: https://github.com/edenhill/librdkafka
[urls.musl_builder_docker_image]: https://github.com/timberio/vector/blob/master/scripts/ci-docker-images/builder-x86_64-unknown-linux-musl/Dockerfile
[urls.openssl]: https://www.openssl.org/
[urls.rust]: https://www.rust-lang.org/