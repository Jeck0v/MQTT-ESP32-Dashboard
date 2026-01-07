/**
 * @typedef {Object} TelemetryData
 * @property {string} deviceId - Device identifier (e.g., 'esp32-01')
 * @property {number} ts - Unix timestamp in seconds
 * @property {number} seq - Sequence number
 * @property {number} tempC - Temperature in Celsius
 * @property {number} humPct - Humidity percentage
 * @property {number} batteryPct - Battery percentage
 * @property {string} topic - MQTT topic
 */

/**
 * @typedef {Object} Statistics
 * @property {number} avgTemp - Average temperature
 * @property {number} avgHum - Average humidity
 * @property {number} count - Total data points
 * @property {number} deviceCount - Number of active devices
 */

/**
 * @typedef {Object} MqttMessage
 * @property {string} type - Message type ('auth', 'auth_response', 'mqtt')
 * @property {string} [key] - Authentication key
 * @property {string} [status] - Status ('success', 'failure')
 * @property {string} [message] - Status message
 * @property {string} [topic] - MQTT topic
 * @property {TelemetryData} [payload] - Message payload
 */

export {};
