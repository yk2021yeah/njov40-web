# get the base image
FROM rust:1.61.0 AS build

# install build dependencies
RUN apt update \
  && apt install -y --no-install-recommends lsb-release apt-transport-https \
  build-essential curl git

# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# clone repo
RUN git clone https://github.com/yk2021yeah/njov40-web

ENV HOST=0.0.0.0
WORKDIR /app/njov40-web
# get trunk
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.15.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

# check project contents
RUN pwd
RUN ls -al

# build app
RUN /app/njov40-web/trunk build --release

# start dev server
CMD /app/njov40-web/trunk serve
