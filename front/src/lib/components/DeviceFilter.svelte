<script>
    /**
     * @typedef {Object} TelemetryData
     * @property {string} deviceId
     * @property {number} ts
     * @property {number} seq
     * @property {number} tempC
     * @property {number} humPct
     * @property {number} batteryPct
     * @property {string} topic
     */

    /** @type {{ selectedDevices: string[], allDevices: string[], telemetryData?: TelemetryData[] }} */
    let { selectedDevices = $bindable(), allDevices, telemetryData = [] } = $props();

    /**
     * @param {string} deviceId
     * @param {boolean} checked
     */
    function toggleDevice(deviceId, checked) {
        if (checked) {
            if (!selectedDevices.includes(deviceId)) {
                selectedDevices = [...selectedDevices, deviceId];
            }
        } else {
            selectedDevices = selectedDevices.filter((/** @type {string} */ id) => id !== deviceId);
        }
    }

    function selectAll() {
        selectedDevices = [...allDevices];
    }

    function clearAll() {
        selectedDevices = [];
    }

    /**
     * @param {string} deviceId
     * @returns {number | null}
     */
    function getLatestBattery(deviceId) {
        const deviceData = telemetryData.filter((/** @type {TelemetryData} */ item) => item.deviceId === deviceId);
        if (deviceData.length === 0) return null;
        const latest = deviceData[0];
        return latest.batteryPct;
    }

    /**
     * @param {number | null} batteryPct
     * @returns {string}
     */
    function getBatteryColor(batteryPct) {
        if (batteryPct === null || batteryPct === undefined) return '#3a3a3a';
        if (batteryPct > 70) return '#00ff80';
        if (batteryPct > 30) return '#ffaa00';
        return '#ff4444';
    }

    /**
     * @param {number | null} batteryPct
     * @returns {string}
     */
    function getBatteryGlow(batteryPct) {
        if (batteryPct === null || batteryPct === undefined) return 'rgba(58, 58, 58, 0.2)';
        if (batteryPct > 70) return 'rgba(0, 255, 128, 0.3)';
        if (batteryPct > 30) return 'rgba(255, 170, 0, 0.3)';
        return 'rgba(255, 68, 68, 0.3)';
    }
</script>

<div class="filter">
    <div class="filter-header">
        <div class="filter-title">DEVICE FILTER</div>
        <div class="filter-count">{selectedDevices.length}/{allDevices.length}</div>
    </div>

    <div class="device-list">
        {#each allDevices as deviceId, index}
            {@const battery = getLatestBattery(deviceId)}
            <label class="device-item">
                <input
                    type="checkbox"
                    checked={selectedDevices.includes(deviceId)}
                    onchange={(e) => {
                        const target = /** @type {HTMLInputElement} */ (e.target);
                        toggleDevice(deviceId, target.checked);
                    }}
                />
                <span class="device-checkbox"></span>
                <div class="device-info">
                    <span class="device-label">{deviceId}</span>
                    <div class="battery-indicator">
                        <div class="battery-container">
                            <div
                                class="battery-bar"
                                style="width: {battery !== null ? battery : 0}%; background: {getBatteryColor(battery)}; box-shadow: 0 0 8px {getBatteryGlow(battery)};"
                            ></div>
                        </div>
                        <span class="battery-text" style="color: {getBatteryColor(battery)};">
                            {battery !== null ? `${battery.toFixed(0)}%` : 'N/A'}
                        </span>
                    </div>
                </div>
            </label>
        {/each}
    </div>

    <div class="filter-actions">
        <button onclick={selectAll} class="filter-btn">SELECT ALL</button>
        <button onclick={clearAll} class="filter-btn">CLEAR ALL</button>
    </div>
</div>

<style>
    .filter {
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        padding: 1.5rem;
    }

    .filter-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
        padding-bottom: 1rem;
        border-bottom: 1px solid #2a2a2a;
    }

    .filter-title {
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 0.15em;
        color: #ffffff;
        text-transform: uppercase;
    }

    .filter-count {
        font-size: 0.875rem;
        font-family: 'JetBrains Mono', monospace;
        color: #00ffff;
        font-variant-numeric: tabular-nums;
    }

    /* Device list */
    .device-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        margin-bottom: 1.5rem;
    }

    .device-item {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
        padding: 1rem 0.875rem;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        border: 1px solid transparent;
        position: relative;
    }

    .device-item:hover {
        background: rgba(255, 255, 255, 0.02);
        border-color: #2a2a2a;
    }

    /* Hide default checkbox */
    .device-item input[type="checkbox"] {
        position: absolute;
        opacity: 0;
        width: 0;
        height: 0;
    }

    /* Custom checkbox */
    .device-checkbox {
        width: 18px;
        height: 18px;
        border: 2px solid #3a3a3a;
        background: transparent;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        position: relative;
        flex-shrink: 0;
        margin-top: 2px;
    }

    .device-item input:checked + .device-checkbox {
        background: #00ffff;
        border-color: #00ffff;
        box-shadow: 0 0 10px rgba(0, 255, 255, 0.3);
    }

    .device-item input:checked + .device-checkbox::after {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 8px;
        height: 8px;
        background: #0a0a0a;
    }

    .device-item:hover .device-checkbox {
        border-color: #666;
    }

    .device-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .device-label {
        font-size: 0.875rem;
        font-family: 'JetBrains Mono', monospace;
        color: #ffffff;
        letter-spacing: 0.05em;
        text-transform: uppercase;
    }

    .device-item input:not(:checked) + .device-checkbox + .device-info .device-label {
        color: #666;
    }

    /* Battery indicator */
    .battery-indicator {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        position: relative;
    }

    .battery-container {
        flex: 1;
        height: 4px;
        background: #2a2a2a;
        border-radius: 2px;
        overflow: hidden;
        position: relative;
    }

    .battery-bar {
        height: 100%;
        background: #00ff80;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        border-radius: 2px;
        min-width: 2px;
    }

    .battery-text {
        font-size: 0.65rem;
        font-family: 'JetBrains Mono', monospace;
        font-weight: 600;
        min-width: 35px;
        text-align: right;
        letter-spacing: 0.05em;
    }

    /* Filter actions */
    .filter-actions {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 0.75rem;
        padding-top: 1rem;
        border-top: 1px solid #2a2a2a;
    }

    .filter-btn {
        padding: 0.75rem;
        font-size: 0.65rem;
        font-weight: 700;
        font-family: inherit;
        letter-spacing: 0.15em;
        text-transform: uppercase;
        background: transparent;
        color: #888;
        border: 1px solid #2a2a2a;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .filter-btn:hover {
        background: rgba(255, 255, 255, 0.05);
        color: #ffffff;
        border-color: #3a3a3a;
    }

    .filter-btn:active {
        background: rgba(255, 255, 255, 0.1);
    }
</style>
