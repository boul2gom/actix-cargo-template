FROM lukemathwalker/cargo-chef:latest-rust-alpine3.16 AS chef

WORKDIR /usr/src/app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --package {{project-name}} --bin {{project-name}}

FROM alpine:3.16.2 AS runtime

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/{{project-name}} .

ENTRYPOINT ["./{{project-name}}"]