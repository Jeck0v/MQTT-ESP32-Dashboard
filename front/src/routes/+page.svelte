<script>
    import DeviceFilter from '$lib/components/DeviceFilter.svelte';
    import StatisticsCards from '$lib/components/StatisticsCards.svelte';
    import TelemetryBarChart from '$lib/components/TelemetryBarChart.svelte';

    /**
     * @typedef {Object} TelemetryRecord
     * @property {string} deviceId
     * @property {number} ts
     * @property {number} seq
     * @property {number} tempC
     * @property {number} humPct
     * @property {number} batteryPct
     * @property {string} topic
     */

    /** @type {WebSocket | null} */
    let ws = $state(null);
    let message = $state('');
    let connected = $state(false);
    let authenticated = $state(false);
    let hasDevices = $derived(allDevices.length > 0);

    // Filter - Dynamic device discovery
    /** @type {string[]} */
    let allDevices = $state([]);
    /** @type {string[]} */
    let selectedDevices = $state([]);

    /** @type {TelemetryRecord[]} */
    let telemetryData = $state([]);
    /** @type {TelemetryRecord | null} */
    let latestTelemetry = $state(null);
    const MAX_MESSAGES = 200;

    let filteredData = $derived(
        telemetryData.filter(item => selectedDevices.includes(item.deviceId))
    );

    let statistics = $derived({
        avgTemp: filteredData.length > 0
            ? filteredData.reduce((sum, d) => sum + d.tempC, 0) / filteredData.length
            : 0,
        avgHum: filteredData.length > 0
            ? filteredData.reduce((sum, d) => sum + d.humPct, 0) / filteredData.length
            : 0,
        count: filteredData.length,
        deviceCount: new Set(filteredData.map(d => d.deviceId)).size,
        totalDevices: selectedDevices.length
    });

    let chartData = $derived(
        filteredData.slice().sort((a, b) => a.ts - b.ts).slice(-50)
    );

    const AUTH_KEY = 'mqtt-classroom-secret-2026';

    function connect() {
        ws = new WebSocket('ws://localhost:8080');

        ws.onopen = () => {
            connected = true;
            message = 'Authenticating...';
            const authMessage = JSON.stringify({ type: 'auth', key: AUTH_KEY });
            if (ws) ws.send(authMessage);
        };

        ws.onmessage = (/** @type {MessageEvent} */ event) => {
            try {
                /** @type {any} */
                const data = JSON.parse(event.data);
                if (data.type === 'auth_response') {
                    if (data.status === 'success') {
                        authenticated = true;
                        message = 'System online';
                    } else {
                        authenticated = false;
                        message = `Auth failed: ${data.message}`;
                        disconnect();
                    }
                } else if (data.type === 'mqtt' && data.payload) {
                    /** @type {TelemetryRecord} */
                    const telemetry = {
                        deviceId: data.payload.deviceId,
                        ts: data.payload.ts,
                        seq: data.payload.seq,
                        tempC: data.payload.tempC,
                        humPct: data.payload.humPct,
                        batteryPct: data.payload.batteryPct,
                        topic: data.topic
                    };

                    // Auto-discover new devices
                    if (!allDevices.includes(telemetry.deviceId)) {
                        allDevices = [...allDevices, telemetry.deviceId].sort();
                        selectedDevices = [...selectedDevices, telemetry.deviceId];
                    }

                    latestTelemetry = telemetry;
                    telemetryData = [telemetry, ...telemetryData].slice(0, MAX_MESSAGES);
                    message = `DATA_RX: ${telemetry.deviceId}`;
                }
            } catch (e) {
                message = `Error parsing data`;
            }
        };

        ws.onclose = () => {
            connected = false;
            authenticated = false;
            message = 'Connection terminated';
        };

        ws.onerror = () => {
            message = 'Connection error';
        };
    }

    function disconnect() {
        if (ws) {
            ws.close();
            ws = null;
        }
    }
</script>

<div class="app">
    <div class="status-bar">
        <div class="status-left">
            <span class="title">MQTT Telemetry System</span>
        </div>
        <div class="status-right">
            <div class="status-indicator" class:active={connected && authenticated}>
                <span class="dot"></span>
                <span class="status-text">
                    {#if !connected}
                        OFFLINE
                    {:else if !authenticated}
                        AUTHENTICATING
                    {:else}
                        ONLINE
                    {/if}
                </span>
            </div>
        </div>
    </div>

    <!-- Main container -->
    <div class="main-container">
        {#if !connected}
            <!-- Connection screen -->
            <div class="connect-screen">
                <div class="connect-content">
                    <div class="system-info">
                        <div class="info-line">SYSTEM STATUS: STANDBY</div>
                        <div class="info-line">MQTT BROKER: ws://localhost:8080</div>
                        <div class="info-line">AUTH KEY: CONFIGURED</div>
                    </div>
                    <button onclick={connect} class="connect-btn">
                        INITIALIZE CONNECTION
                    </button>
                </div>
            </div>
        {:else if !authenticated}
            <!-- Authenticating screen -->
            <div class="connect-screen">
                <div class="connect-content">
                    <div class="system-info">
                        <div class="info-line">ESTABLISHING SECURE CONNECTION</div>
                        <div class="info-line loading">AUTHENTICATING...</div>
                    </div>
                </div>
            </div>
        {:else}
            <!-- Dashboard -->
            <div class="dashboard">
                <!-- Left sidebar -->
                <div class="sidebar">
                    {#if hasDevices}
                        <DeviceFilter bind:selectedDevices {allDevices} {telemetryData} />
                    {:else}
                        <div class="no-devices-panel">
                            <div class="no-devices-header">
                                <div class="filter-title">DEVICE FILTER</div>
                                <div class="filter-count">0/0</div>
                            </div>
                            <div class="no-devices-content">
                                <div class="no-devices-icon">
                                    <div class="pulse-ring"></div>
                                    <div class="pulse-ring delay-1"></div>
                                    <div class="pulse-ring delay-2"></div>
                                </div>
                                <div class="no-devices-text">NO DEVICES DETECTED</div>
                                <div class="no-devices-hint">Waiting for ESP32 telemetry...</div>
                            </div>
                        </div>
                    {/if}

                    <div class="message-log">
                        <div class="log-header">SYSTEM LOG</div>
                        <div class="log-content">{message || 'Awaiting telemetry'}</div>
                    </div>
                </div>

                <!-- Main content -->
                <div class="content">
                    {#if !hasDevices}
                        <div class="waiting-screen">
                            <div class="waiting-content">
                                <div class="scanning-animation">
                                    <div class="scan-line"></div>
                                    <div class="scan-text">SCANNING FOR DEVICES</div>
                                </div>
                                <div class="waiting-info">
                                    <div class="info-item">
                                        <span class="info-label">STATUS:</span>
                                        <span class="info-value">LISTENING</span>
                                    </div>
                                    <div class="info-item">
                                        <span class="info-label">TOPIC:</span>
                                        <span class="info-value">classroom/+/telemetry</span>
                                    </div>
                                    <div class="info-item">
                                        <span class="info-label">DEVICES FOUND:</span>
                                        <span class="info-value">0</span>
                                    </div>
                                </div>
                                <div class="waiting-hint">
                                    Ensure ESP32 devices are powered on and publishing to MQTT broker
                                </div>
                            </div>
                        </div>
                    {:else}
                        <StatisticsCards {statistics} />
                        {#if filteredData.length > 0}
                            <TelemetryBarChart {chartData} />
                        {:else}
                            <div class="no-data">
                                <div class="no-data-text">NO DATA AVAILABLE</div>
                                <div class="no-data-hint">Select devices to begin monitoring</div>
                            </div>
                        {/if}
                    {/if}
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family: 'JetBrains Mono', 'Roboto Mono', 'Courier New', monospace;
        background: #0a0a0a;
        color: #ffffff;
        overflow-x: hidden;
    }

    :global(*) {
        box-sizing: border-box;
    }

    .app {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        animation: fadeIn 0.6s cubic-bezier(0.4, 0, 0.2, 1);
    }

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    .status-bar {
        position: sticky;
        top: 0;
        z-index: 100;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem 2rem;
        background: #0a0a0a;
        border-bottom: 1px solid #2a2a2a;
        animation: slideDown 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    }

    @keyframes slideDown {
        from { transform: translateY(-100%); }
        to { transform: translateY(0); }
    }

    .status-left .title {
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 0.2em;
        color: #ffffff;
        text-transform: uppercase;
    }

    .status-indicator {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.5rem 1rem;
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .status-indicator.active {
        border-color: #00ffff;
        box-shadow: 0 0 20px rgba(0, 255, 255, 0.2);
    }

    .dot {
        width: 8px;
        height: 8px;
        background: #666;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .status-indicator.active .dot {
        background: #00ffff;
        box-shadow: 0 0 10px #00ffff;
        animation: pulse 2s infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }

    .status-text {
        font-size: 0.75rem;
        font-weight: 600;
        letter-spacing: 0.15em;
        color: #888;
    }

    .status-indicator.active .status-text {
        color: #00ffff;
    }

    .main-container {
        flex: 1;
        padding: 2rem;
    }

    .connect-screen {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: calc(100vh - 4rem - 4rem);
        animation: fadeIn 0.8s cubic-bezier(0.4, 0, 0.2, 1) 0.2s both;
    }

    .connect-content {
        text-align: center;
        max-width: 600px;
    }

    .system-info {
        margin-bottom: 3rem;
        padding: 2rem;
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
    }

    .info-line {
        font-size: 0.875rem;
        letter-spacing: 0.1em;
        color: #888;
        margin: 0.75rem 0;
        text-transform: uppercase;
    }

    .info-line.loading {
        color: #00ffff;
        animation: blink 1.5s infinite;
    }

    @keyframes blink {
        0%, 49% { opacity: 1; }
        50%, 100% { opacity: 0.3; }
    }

    .connect-btn {
        padding: 1.25rem 3rem;
        font-size: 0.875rem;
        font-weight: 700;
        font-family: inherit;
        letter-spacing: 0.2em;
        text-transform: uppercase;
        background: transparent;
        color: #ffffff;
        border: 2px solid #ffffff;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .connect-btn:hover {
        background: #ffffff;
        color: #0a0a0a;
        box-shadow: 0 0 30px rgba(255, 255, 255, 0.3);
        transform: translateY(-2px);
    }

    .connect-btn:active {
        transform: translateY(0);
    }

    .dashboard {
        display: grid;
        grid-template-columns: 320px 1fr;
        gap: 2rem;
        min-height: calc(100vh - 4rem - 4rem);
        animation: fadeIn 0.8s cubic-bezier(0.4, 0, 0.2, 1) 0.2s both;
    }

    .sidebar {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        animation: slideRight 0.6s cubic-bezier(0.4, 0, 0.2, 1) 0.3s both;
    }

    @keyframes slideRight {
        from { transform: translateX(-50px); opacity: 0; }
        to { transform: translateX(0); opacity: 1; }
    }

    .message-log {
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        padding: 1.5rem;
    }

    .log-header {
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 0.15em;
        color: #666;
        margin-bottom: 1rem;
        text-transform: uppercase;
    }

    .log-content {
        font-size: 0.75rem;
        color: #00ffff;
        font-family: 'JetBrains Mono', monospace;
        word-break: break-all;
    }

    .content {
        display: flex;
        flex-direction: column;
        gap: 2rem;
        animation: slideLeft 0.6s cubic-bezier(0.4, 0, 0.2, 1) 0.4s both;
    }

    @keyframes slideLeft {
        from { transform: translateX(50px); opacity: 0; }
        to { transform: translateX(0); opacity: 1; }
    }

    .no-data {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 400px;
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
    }

    .no-data-text {
        font-size: 1.5rem;
        font-weight: 700;
        letter-spacing: 0.2em;
        color: #666;
        margin-bottom: 1rem;
    }

    .no-data-hint {
        font-size: 0.875rem;
        color: #444;
        letter-spacing: 0.1em;
    }

    /* No devices panel */
    .no-devices-panel {
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        padding: 1.5rem;
    }

    .no-devices-header {
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
        color: #666;
        font-variant-numeric: tabular-nums;
    }

    .no-devices-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 3rem 1rem;
        gap: 1.5rem;
    }

    .no-devices-icon {
        position: relative;
        width: 80px;
        height: 80px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .pulse-ring {
        position: absolute;
        width: 100%;
        height: 100%;
        border: 2px solid #00ffff;
        border-radius: 50%;
        opacity: 0;
        animation: pulse-animation 3s cubic-bezier(0.4, 0, 0.2, 1) infinite;
    }

    .pulse-ring.delay-1 {
        animation-delay: 1s;
    }

    .pulse-ring.delay-2 {
        animation-delay: 2s;
    }

    @keyframes pulse-animation {
        0% {
            transform: scale(0.3);
            opacity: 1;
        }
        50% {
            opacity: 0.3;
        }
        100% {
            transform: scale(1.2);
            opacity: 0;
        }
    }

    .no-devices-text {
        font-size: 0.875rem;
        font-weight: 700;
        letter-spacing: 0.2em;
        color: #666;
        text-transform: uppercase;
    }

    .no-devices-hint {
        font-size: 0.75rem;
        color: #444;
        letter-spacing: 0.1em;
        text-align: center;
        animation: blink 2s infinite;
    }

    /* Waiting screen */
    .waiting-screen {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 500px;
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        padding: 3rem;
    }

    .waiting-content {
        max-width: 500px;
        width: 100%;
        text-align: center;
    }

    .scanning-animation {
        position: relative;
        margin-bottom: 3rem;
        padding: 2rem 0;
    }

    .scan-line {
        width: 100%;
        height: 2px;
        background: linear-gradient(90deg,
            transparent 0%,
            #00ffff 50%,
            transparent 100%);
        animation: scan 2s ease-in-out infinite;
        box-shadow: 0 0 20px rgba(0, 255, 255, 0.5);
    }

    @keyframes scan {
        0%, 100% {
            transform: translateX(-100%);
            opacity: 0;
        }
        50% {
            opacity: 1;
        }
        100% {
            transform: translateX(100%);
        }
    }

    .scan-text {
        font-size: 1rem;
        font-weight: 700;
        letter-spacing: 0.2em;
        color: #00ffff;
        text-transform: uppercase;
        margin-top: 1.5rem;
        animation: blink 1.5s infinite;
    }

    .waiting-info {
        background: rgba(0, 255, 255, 0.05);
        border: 1px solid #2a2a2a;
        padding: 2rem;
        margin-bottom: 2rem;
    }

    .info-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem 0;
        border-bottom: 1px solid #2a2a2a;
    }

    .info-item:last-child {
        border-bottom: none;
    }

    .info-label {
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 0.15em;
        color: #666;
        text-transform: uppercase;
    }

    .info-value {
        font-size: 0.875rem;
        font-family: 'JetBrains Mono', monospace;
        color: #00ffff;
        font-weight: 600;
    }

    .waiting-hint {
        font-size: 0.75rem;
        color: #666;
        letter-spacing: 0.05em;
        line-height: 1.6;
    }

    @media (max-width: 1024px) {
        .dashboard {
            grid-template-columns: 1fr;
        }

        .sidebar {
            order: 2;
        }

        .content {
            order: 1;
        }
    }

    @media (max-width: 768px) {
        .status-bar {
            padding: 1rem;
            flex-direction: column;
            align-items: flex-start;
            gap: 0.75rem;
        }

        .main-container {
            padding: 1rem;
        }

        .waiting-screen {
            padding: 2rem 1rem;
        }

        .waiting-info {
            padding: 1.5rem;
        }

        .info-item {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.5rem;
        }
    }
</style>
