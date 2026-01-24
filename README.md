# CarBot

## Build docker

Move to rpi: `cd rpi`. Then, use `docker compose up --build`.

## Test USB connection

Try to send commands to the Arduino with `screen /dev/ttyUSB0 115200`.

Allow the user to access USB: `sudo usermod -a -G dialout $USER`.