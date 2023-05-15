FROM alpine:latest as build 

ENV LANG zh_CN.UTF-8
ENV LC_ALL zh_CN.UTF-8
ENV LANGUAGE zh_CN.UTF-8
ENV TZ Asia/Shanghai
ENV TERM xterm-256color

RUN apk update && apk add curl
 
RUN curl https://sh.rustup.rs -sSf \
  | sh -s -- -y --no-modify-path --default-toolchain nightly

RUN apk add --no-cache gcc g++ git bash cmake make pkgconfig openssl-dev libc6-compat clang nasm util-linux

SHELL ["/bin/bash","-c"]

ADD sh/upx.install.sh sh/upx.install.sh
RUN ./sh/upx.install.sh

WORKDIR /root

ADD sh/cflag.sh sh/cflag.sh
# ADD sh/jpegxl-rs.sh sh/jpegxl-rs.sh

# RUN ./sh/jpegxl-rs.sh

ADD src src
ADD Cargo.toml .
ADD dist.sh .
ADD sh/ sh/

RUN ./dist.sh && ./sh/cpso.sh && ./sh/upx.sh && mv target/app / && rm -rf target

FROM scratch

ENV RUST_LOG=debug,supervisor=warn,hyper=warn,rustls=warn,quinn_udp=warn
COPY --from=build /app .

ENV LD_LIBRARY_PATH=/lib
COPY --from=build /so/ /lib/

ENV RUST_BACKTRACE=short

CMD ["/app"]
