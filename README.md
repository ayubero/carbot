# CarBot

## Build for Raspberry Pi

Navigate to the backend directory `cd ./backend` and build with `docker buildx build --platform linux/arm64 -t eindres/backend:latest --push .`.

For the frontend, navigate using `cd ./frotend` and build with `docker buildx build --platform linux/arm64 -t eindres/frontend:latest --push .`. 

## On the Raspberry Pi

Login to download the image with `docker login`, pull the images with `docker compose pull` and launch the containers using `docker compose up -d`.

## Test USB connection

Try to send commands to the Arduino with `screen /dev/ttyUSB0 115200`.

Allow the user to access USB: `sudo usermod -a -G dialout $USER`.