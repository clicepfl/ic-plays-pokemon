```sh
docker build -t ipp .
docker run -p 6901:6901 -e VNC_VIEW_ONLY=true -v $PWD/.config:/headless/.config ipp
```
