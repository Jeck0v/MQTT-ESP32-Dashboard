<script>
    let { statistics } = $props();
</script>

<div class="metrics">
    <!-- Hero numbers -->
    <div class="hero-metrics">
        <div class="hero-metric">
            <div class="metric-label">TEMPERATURE MOY</div>
            <div class="metric-value">
                <span class="number">{statistics.avgTemp.toFixed(1)}</span>
                <span class="unit">°C</span>
            </div>
        </div>

        <div class="hero-metric">
            <div class="metric-label">HUMIDITY MOY</div>
            <div class="metric-value">
                <span class="number">{statistics.avgHum.toFixed(1)}</span>
                <span class="unit">%</span>
            </div>
        </div>
    </div>

    <!-- Secondary metrics -->
    <div class="secondary-metrics">
        <div class="metric-item">
            <div class="metric-item-label">ACTIVE DEVICES</div>
            <div class="metric-item-value">{statistics.deviceCount}{#if statistics.totalDevices} / {statistics.totalDevices}{/if}</div>
        </div>

        <div class="metric-divider"></div>

        <div class="metric-item">
            <div class="metric-item-label">DATA POINTS</div>
            <div class="metric-item-value">{statistics.count}</div>
        </div>
    </div>
</div>

<style>
    .metrics {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        animation: fadeIn 0.8s cubic-bezier(0.4, 0, 0.2, 1) 0.5s both;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(20px); }
        to { opacity: 1; transform: translateY(0); }
    }

    .hero-metrics {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 1.5rem;
    }

    .hero-metric {
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        padding: 3rem 2rem;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        position: relative;
        overflow: hidden;
    }

    .hero-metric::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: #00ffff;
        transform: scaleX(0);
        transform-origin: left;
        transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .hero-metric:hover::before {
        transform: scaleX(1);
    }

    .hero-metric:hover {
        border-color: #3a3a3a;
        box-shadow: 0 0 40px rgba(0, 255, 255, 0.1);
        transform: translateY(-4px);
    }

    .metric-label {
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 0.2em;
        color: #666;
        text-transform: uppercase;
    }

    .metric-value {
        display: flex;
        align-items: baseline;
        gap: 0.5rem;
    }

    .number {
        font-size: clamp(4rem, 8vw, 8rem);
        font-weight: 700;
        font-family: 'JetBrains Mono', 'Roboto Mono', monospace;
        line-height: 1;
        color: #ffffff;
        font-variant-numeric: tabular-nums;
        letter-spacing: -0.02em;
        text-shadow: 0 0 40px rgba(0, 255, 255, 0.3);
    }

    .unit {
        font-size: 2rem;
        font-weight: 400;
        color: #666;
        align-self: flex-end;
        margin-bottom: 0.5rem;
    }

    /* Secondary metrics */
    .secondary-metrics {
        display: flex;
        align-items: center;
        background: #1a1a1a;
        border: 1px solid #2a2a2a;
        padding: 1.5rem 2rem;
        gap: 2rem;
    }

    .metric-item {
        flex: 1;
    }

    .metric-item-label {
        font-size: 0.65rem;
        font-weight: 700;
        letter-spacing: 0.15em;
        color: #666;
        text-transform: uppercase;
        margin-bottom: 0.75rem;
    }

    .metric-item-value {
        font-size: 1.5rem;
        font-weight: 700;
        font-family: 'JetBrains Mono', monospace;
        color: #ffffff;
        font-variant-numeric: tabular-nums;
    }

    .metric-divider {
        width: 1px;
        height: 3rem;
        background: #2a2a2a;
    }

    /* Responsive */
    @media (max-width: 768px) {
        .hero-metrics {
            grid-template-columns: 1fr;
        }

        .hero-metric {
            padding: 2rem 1.5rem;
        }

        .number {
            font-size: clamp(3rem, 12vw, 6rem);
        }

        .unit {
            font-size: 1.5rem;
        }

        .secondary-metrics {
            flex-direction: column;
            align-items: stretch;
            gap: 1.5rem;
            padding: 1.5rem;
        }

        .metric-divider {
            width: 100%;
            height: 1px;
        }
    }
</style>
