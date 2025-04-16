FROM ubuntu:22.04

# Install X11, GNOME dependencies and Nautilus
RUN apt-get update && apt-get install -y \
    libcanberra-gtk3-module \
    dbus-x11 \
    x11-apps \
    --no-install-recommends

RUN apt-get -y install wget unzip
RUN apt-get -y install libsdl2-dev

# Set environment variables for display
ENV DISPLAY=:0

# Create a new user to avoid running as root
RUN useradd -ms /bin/bash user
USER user
WORKDIR /home/user

RUN wget https://dlf.emu-land.net/nes/Mesen%20v2.0.0%2C%208%20Jul%202024%20%28Linux%20-%20ubuntu-20.04%20-%20clang_aot%29.zip -O Mesen.zip
RUN unzip -d . Mesen.zip
RUN chmod +x ./Mesen

COPY PokemonRed.zip /home/user/PokemonRed.zip

# Command to run Nautilus
CMD ["./Mesen", "PokemonRed.zip"]
