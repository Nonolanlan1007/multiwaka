FROM rust:1-bullseye AS base

RUN apt-get update && apt-get install -y pkg-config libssl-dev g++ lld clang && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

FROM base AS build
COPY . .
RUN cargo build --release

FROM base AS final
COPY --from=build /usr/src/app/target/release/multi_waka ./multi_waka

ENV MULTIWAKA_PORT=8080

LABEL "org.opencontainers.image.created"="2026-01-27T09:55:47.000Z"
LABEL "org.opencontainers.image.authors"="Nolhan"
LABEL "org.opencontainers.image.url"="https://github.com/Nonolanlan1007/multiwaka"
LABEL "org.opencontainers.image.source"="https://github.com/Nonolanlan1007/multiwaka"
LABEL "org.opencontainers.image.version"="1.1.0"
LABEL "org.opencontainers.image.licenses"="GNU GPLv3"
LABEL "org.opencontainers.image.title"="MultiWaka"
LABEL "org.opencontainers.image.description"="A simple software that allows you to use multiple wakatime instances at the same time"

EXPOSE 8080

CMD ["./multi_waka"]