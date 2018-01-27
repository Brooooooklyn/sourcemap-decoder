# Docker template for Chinese user

FROM node:8.9.4-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

ADD sources.list /etc/apt/sources.list

RUN apt-get update && apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    wget \
    software-properties-common -y && \
    apt-get autoremove -y && apt-get purge && apt-get autoclean

RUN set -eux; \
    \
    dpkgArch="$(dpkg --print-architecture)"; \
	case "${dpkgArch##*-}" in \
		amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='4b7a67cd971d713e0caef48b5754190aca19192d1863927a005c3432512b12dc' ;; \
		armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='622190c3f478a56563d45f6fbc1fab02d356b631c28a1beba2c3e4c68de3c14c' ;; \
		arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='a39d7643cdced9ad70a9927bbb0a861b579884f94793881b771d3a0f92c0ddd8' ;; \
		i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='9e921fce4a2cc1f04095be6d623effdead0aab1261472e6933da9e6030330b90' ;; \
		*) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
	esac; \
    \
    url="https://mirrors.ustc.edu.cn/rust-static/rustup/archive/1.9.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    export RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static";\
    export RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup";\
    ./rustup-init -y --no-modify-path --default-toolchain 1.23.0; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;
