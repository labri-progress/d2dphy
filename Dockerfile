# building
FROM rust:bullseye as builder
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    clang libclang-dev apt-utils libudev-dev curl && \
    rm -rf /var/lib/apt/lists/*

# we copy libphysec for bindgen to be able to find it
# required as we get libphysec from "../../"
RUN mkdir -p /root/STM32PlatformCode/Firmware/SubGHz_Phy/App/
WORKDIR /root
COPY ./STM32PlatformCode/ ./STM32PlatformCode/

RUN mkdir /root/foo/
WORKDIR /root/foo/

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/download/v0.27.0/probe-rs-tools-installer.sh | sh

COPY ./tools/stm32-cli ./stm32-cli
COPY ./tools/stm32-flash ./stm32-flash

WORKDIR /root/foo/stm32-cli
RUN cargo build --release 
WORKDIR /root/foo/stm32-flash
RUN cargo build --release

# runnning
FROM debian:unstable 

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    udev libudev1 curl ca-certificates libc6 usbutils && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /root/foo
COPY ./firmwares/physec-firmware.elf .

# copy the built binaries from the builder stage
COPY --from=builder /root/foo/stm32-cli/target/release/stm32-cli ./stm32-config
COPY --from=builder /root/foo/stm32-flash/target/release/stm32-flash ./stm32-flash

# run flashing command by default, can be change by specifying command line.
# the interactive flag `-i` need to be set for the menus to work as expected
CMD ./stm32-flash -e ./physec-firmware.elf 
