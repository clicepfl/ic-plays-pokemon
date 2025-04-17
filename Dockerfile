FROM rust:1.86.0-slim-bullseye as BUILDER

COPY Cargo.* ./
COPY src ./src/

RUN cargo build --release

FROM consol/rocky-xfce-vnc as RUNNER

USER root
RUN dnf -y install unzip SDL2.x86_64 xdotool

USER 1000

RUN wget https://dlf.emu-land.net/nes/Mesen%20v2.0.0%2C%208%20Jul%202024%20%28Linux%20-%20ubuntu-20.04%20-%20clang_aot%29.zip -O Mesen.zip
RUN unzip -d . Mesen.zip
RUN chmod +x ./Mesen

COPY --from=BUILDER --chown=1000:1000 /target/release/ic-plays-pokemon /headless/server
