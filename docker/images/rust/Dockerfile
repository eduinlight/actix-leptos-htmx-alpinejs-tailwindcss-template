ARG RUST_VERSION
FROM rust:$RUST_VERSION-alpine

ARG UID
ARG GID
RUN if [ -z "`getent group $GID`" ]; then \
      addgroup -S -g $GID user; \
    else \
      groupmod -n user `getent group $GID | cut -d: -f1`; \
    fi \
  && if [ -z "`getent passwd $UID`" ]; then \
      adduser -S -u $UID -G user -s /bin/sh user; \
    else \
      usermod -l user -g $GID -d /home/user -m `getent passwd $UID | cut -d: -f1`; \
    fi \
  && chown -R user:user /home/user

RUN cargo install cargo-watch
