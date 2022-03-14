FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends build-essential pkg-config cmake git curl zlib1g-dev libuv1-dev libssl-dev openssl

WORKDIR /tmp
RUN curl -LO "https://github.com/datastax/cpp-driver/archive/refs/tags/2.16.0.tar.gz" \
    && tar -xzf 2.16.0.tar.gz \
    && rm -rf 2.16.0.tar.gz

RUN cd cpp-driver-2.16.0 \
    && mkdir build \
    && cd build \
    && cmake .. \
    && make \
    && make install

ENV LD_LIBRARY_PATH=/usr/local/lib/x86_64-linux-gnu/
