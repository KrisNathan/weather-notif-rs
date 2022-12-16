Server Usage:

```sh
ADDRESS=127.0.0.1 PORT=8080 TOPIC=YOURNTFYTOPIC cargo run --release
```

Call Job:

```sh
curl http://server/weather/x/y
```

- x: the x position of the pixel to check
- y: the y position of the pixel to check 