FROM registry.cn-beijing.aliyuncs.com/pox/vmchain-final-dev:SNAPSHOT-8

#ARG DEBIAN_FRONTEND=noninteractive
#ENV TZ=Asia/Shanghai
#ENV LANG=C.UTF-8
#
## Install build dependencies
#RUN apt-get update -y \
#    && apt-get install -y automake \
#    build-essential \
#    apt-utils \
#    curl \
#    && apt-get clean \
#RUN #git config --global url."https://github.com.cnpmjs.org/".insteadOf https://github.com/
#RUN curl https://getsubstrate.io -sSf | bash -s -- --fast
#RUN mkdir -p /root/setup
#WORKDIR /root/setup
#ENV PATH=/root/.cargo/bin:$PATH
#RUN cargo --version
#RUN rustup update nightly & \
#    rustup update stable &\
#    rustup target add wasm32-unknown-unknown --toolchain nightly

WORKDIR /apps
COPY . .
#RUN apt-get update;
#RUN apt-get install git

#RUN cargo clean
#RUN cargo update -p parity-db  -p cid
#RUN cargo update -p tracing-subscriber --precise 0.2.19
RUN RUST_BACKTRACE=1 cargo build --release
# Expose ports
RUN ./target/release/vmchain build-spec --chain=customSpec.json --raw --disable-default-bootnode > config/customSpecRaw.json

#FROM nginx:stable-alpine
COPY nginx/nginx.conf /etc/nginx/nginx.conf
EXPOSE 9900
