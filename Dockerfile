FROM ubuntu:xenial

RUN apt-get update && apt-get install -y -q \
  build-essential \
  make \
  openssl \
  python3 \
  gcc \
  man-db \
  libssl-dev

RUN apt-get install -y -q \
    golang \
    curl \
 && curl https://sh.rustup.rs -sSf > /usr/bin/rustup-init \
 && chmod +x /usr/bin/rustup-init \
 && rustup-init -y

RUN apt-get update && apt-get install -y -q \
    python3-pip \
 && pip3 install cffi

ENV PATH=$PATH:/root/.cargo/bin

WORKDIR /project

CMD ./build
