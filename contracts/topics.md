# Topic & Payload

Prefix: `classroom/<deviceId>/`

| Topic  | Sens | Émis par | Payload |
| ------------- |:-------------:|:-------------:|:-------------:|
|`telemetry` | mesures | ESP32 | JSON telemetry
|`events` | events (boot, ack, alert) | ESP32 | JSON event
|`cmd` | commandes() | server | JSON cmd
|`status` | status | ESP32 | JSON status
