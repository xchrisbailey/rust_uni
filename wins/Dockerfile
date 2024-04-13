ARG RUST_VERSION=1.77.2

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

RUN USER=root cargo new --bin wins
WORKDIR /wins
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release 

RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/wins*
RUN cargo build --release

FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

RUN echo ${PWD} && ls -lR

COPY --from=builder /wins/target/release/wins ${APP}/wins
COPY --from=builder /wins/templates ${APP}/templates

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./wins"]
