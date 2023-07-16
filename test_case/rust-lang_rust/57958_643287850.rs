
# syntax = docker/dockerfile:1-experimental
FROM rust:1.43-slim-buster as builder

RUN rm -f /etc/apt/apt.conf.d/docker-clean && \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache
RUN --mount=type=cache,id=dvca,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=dvla,target=/var/lib/apt,sharing=locked \
    apt update && \
    apt install -y build-essential libssl-dev cmake pkg-config lld
