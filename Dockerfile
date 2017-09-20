FROM ubuntu:xenial

RUN apt-get update

RUN apt-get install -y -q \
  build-essential \
  make \
  openssl \
  gcc \
  man-db \
  libssl-dev

RUN apt-get install -y -q \
    golang \
    curl \
 && curl https://sh.rustup.rs -sSf > /usr/bin/rustup-init \
 && chmod +x /usr/bin/rustup-init \
 && rustup-init -y

WORKDIR /project

CMD make
