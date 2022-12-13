Server Usage:

```sh
TOPIC=YOURNTFYTOPIC cargo run --release
```

Call Job:

```sh
curl http://server/weather/x/y
```

- x: the x position of the pixel to check
- y: the y position of the pixel to check 