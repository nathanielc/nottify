# nottify
Simple Desktop Notifications from MQTT

Install and run nottify:

```
nottify --mqtt-url mqtt://localhost --topic $(hostname)/notify
```

Any text sent to the `$(hostname)/notify` topic will display as a desktop notification.

