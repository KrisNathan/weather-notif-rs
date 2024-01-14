Server Usage:

```sh
ADDRESS=127.0.0.1 PORT=8080 TOPIC=YOURNTFYTOPIC cargo run --bin server --release
```

Call Job:

```sh
curl http://server/weather/x/y
```

```sh
curl http://127.0.0.1:8080/weather/332/332
```

- x: the x position of the pixel to check
- y: the y position of the pixel to check

Cron Usage:

```sh
URL=NOTIFSERVERURL cargo run --bin cron --release
``` 

```sh
URL=http://127.0.0.1:8080/weather/332/332 cargo run --bin cron --release
```
