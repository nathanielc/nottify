# nottify
Simple Desktop Notifications from MQTT

Install and run nottify from the repo root:

```
cargo install --locked --path .
nottify --mqtt-url mqtt://localhost --topic $(hostname)/$(whoami)/nottify
```

Any text sent to the `$(hostname)/$(whoami)/nottify` topic will display as a desktop notification.

## Systemd User Service

Create a simple user service to launch and run the notify daemon on boot.

```
# ~/.config/systemd/user/nottify.service
[Unit]
Description=Nottify, simple desktop notifications from MQTT

[Service]
ExecStart=%h/.cargo/bin/nottify --mqtt-url mqtt://localhost --topic %H/%u/nottify
Restart=on-failure

[Install]
WantedBy=default.target
```

