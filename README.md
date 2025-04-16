# IC Plays Pokemon

```sh
# Build the image
docker build -t ipp .

# Run the docker
docker run -p 6901:6901 -e VNC_VIEW_ONLY=true -v $PWD/.config:/headless/.config ipp
```

Can be accessed through [http://localhost:6901/?password=vncpassword](http://localhost:6901/?password=vncpassword).

Keystrokes can be sent by running `xdotool key <key>` in the docker container (e.g. `xdotool key k`).
