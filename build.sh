cargo build --release &&
docker build -f Server.Dockerfile . -t weather-notif-server/krisnathan:latest
docker build -f Cron.Dockerfile . -t weather-notif-cron/krisnathan:latest

