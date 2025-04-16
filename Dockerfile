FROM consol/rocky-xfce-vnc

USER root
RUN dnf -y install unzip SDL2.x86_64

USER 1000

RUN wget https://dlf.emu-land.net/nes/Mesen%20v2.0.0%2C%208%20Jul%202024%20%28Linux%20-%20ubuntu-20.04%20-%20clang_aot%29.zip -O Mesen.zip
RUN unzip -d . Mesen.zip
RUN chmod +x ./Mesen

COPY PokemonRed.zip ./PokemonRed.zip

CMD ["./Mesen", "./PokemonRed.zip"]
