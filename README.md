# CarBot

## Backend

Rust backend needs libudev developer headers:

```sudo apt update
sudo apt install libudev-dev```

Try to send a message to Arduino:

```curl -X POST http://localhost:3000/send \
  -H "Content-Type: application/json" \
  -d '{"port_path":"/dev/ttyACM0","message":"Hello Arduino!\n"}'```