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

	/** @type {{ chartData: TelemetryData[] }} */
	let { chartData } = $props();

	const CHART_HEIGHT = 300;
	const CHART_PADDING = { top: 20, right: 60, bottom: 40, left: 60 };

	let metrics = $derived({
		tempMin: chartData.length > 0 ? Math.min(...chartData.map((d) => d.tempC)) : 0,
		tempMax: chartData.length > 0 ? Math.max(...chartData.map((d) => d.tempC)) : 30,
		humMin: chartData.length > 0 ? Math.min(...chartData.map((d) => d.humPct)) : 0,
		humMax: chartData.length > 0 ? Math.max(...chartData.map((d) => d.humPct)) : 100
	});

	/**
	 * Scale temperature value to chart coordinates
	 * @param {number} value
	 * @returns {number}
	 */
	function scaleTemp(value) {
		const range = metrics.tempMax - metrics.tempMin || 1;
		const chartHeight = CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom;
		return (
			CHART_HEIGHT -
			CHART_PADDING.bottom -
			((value - metrics.tempMin) / range) * chartHeight
		);
	}

	/**
	 * Scale humidity value to chart coordinates
	 * @param {number} value
	 * @returns {number}
	 */
	function scaleHum(value) {
		const range = metrics.humMax - metrics.humMin || 1;
		const chartHeight = CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom;
		return (
			CHART_HEIGHT -
			CHART_PADDING.bottom -
			((value - metrics.humMin) / range) * chartHeight
		);
	}

	/**
	 * Calculate X position for data point
	 * @param {number} index
	 * @param {number} total
	 * @param {number} width
	 * @returns {number}
	 */
	function getX(index, total, width) {
		const chartWidth = width - CHART_PADDING.left - CHART_PADDING.right;
		return CHART_PADDING.left + (index / (total - 1 || 1)) * chartWidth;
	}

	/**
	 * Format timestamp for display
	 * @param {number} ts
	 * @returns {string}
	 */
	function formatTime(ts) {
		const date = new Date(ts * 1000);
		return date.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
	}

	/**
	 * Get device color
	 * @param {string} deviceId
	 * @returns {string}
	 */
	function getDeviceColor(deviceId) {
		const colors = ['#00ffff', '#ff00ff', '#ffff00', '#00ff00', '#ff8800'];
		const hash = deviceId.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
		return colors[hash % colors.length];
	}
</script>

<div class="chart-container">
	<div class="chart-header">
		<div class="chart-title">TELEMETRY DATA</div>
		<div class="chart-legend">
			<div class="legend-item">
				<span class="legend-dot" style="background: #00ffff;"></span>
				<span class="legend-label">TEMPERATURE</span>
			</div>
			<div class="legend-item">
				<span class="legend-dot" style="background: #ff00ff;"></span>
				<span class="legend-label">HUMIDITY</span>
			</div>
		</div>
	</div>

	<div class="chart-wrapper">
		{#if chartData.length === 0}
			<div class="chart-empty">
				<div class="empty-text">NO DATA AVAILABLE</div>
			</div>
		{:else}
			<svg class="chart-svg" viewBox="0 0 800 {CHART_HEIGHT}" preserveAspectRatio="none">
				<!-- Grid lines -->
				<g class="grid">
					{#each Array(5) as _, i}
						{@const y = CHART_PADDING.top + (i * (CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom)) / 4}
						<line
							x1={CHART_PADDING.left}
							y1={y}
							x2={800 - CHART_PADDING.right}
							y2={y}
							stroke="#2a2a2a"
							stroke-width="1"
						/>
					{/each}
				</g>

				<!-- Temperature line -->
				<g class="temp-line">
					{#each chartData as point, i}
						{#if i > 0}
							{@const prevPoint = chartData[i - 1]}
							<line
								x1={getX(i - 1, chartData.length, 800)}
								y1={scaleTemp(prevPoint.tempC)}
								x2={getX(i, chartData.length, 800)}
								y2={scaleTemp(point.tempC)}
								stroke="#00ffff"
								stroke-width="2"
							/>
						{/if}
					{/each}
					{#each chartData as point, i}
						<circle
							cx={getX(i, chartData.length, 800)}
							cy={scaleTemp(point.tempC)}
							r="3"
							fill="#00ffff"
							class="data-point"
						>
							<title>{point.deviceId}: {point.tempC.toFixed(1)}°C</title>
						</circle>
					{/each}
				</g>

				<!-- Humidity line -->
				<g class="hum-line">
					{#each chartData as point, i}
						{#if i > 0}
							{@const prevPoint = chartData[i - 1]}
							<line
								x1={getX(i - 1, chartData.length, 800)}
								y1={scaleHum(prevPoint.humPct)}
								x2={getX(i, chartData.length, 800)}
								y2={scaleHum(point.humPct)}
								stroke="#ff00ff"
								stroke-width="2"
							/>
						{/if}
					{/each}
					{#each chartData as point, i}
						<circle
							cx={getX(i, chartData.length, 800)}
							cy={scaleHum(point.humPct)}
							r="3"
							fill="#ff00ff"
							class="data-point"
						>
							<title>{point.deviceId}: {point.humPct.toFixed(1)}%</title>
						</circle>
					{/each}
				</g>

				<!-- Y-axis labels (Temperature) -->
				<g class="y-axis-temp">
					{#each Array(5) as _, i}
						{@const value = metrics.tempMin + (i * (metrics.tempMax - metrics.tempMin)) / 4}
						{@const y = CHART_HEIGHT - CHART_PADDING.bottom - (i * (CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom)) / 4}
						<text x={CHART_PADDING.left - 10} y={y} class="axis-label" text-anchor="end">
							{value.toFixed(0)}°C
						</text>
					{/each}
				</g>

				<!-- Y-axis labels (Humidity) -->
				<g class="y-axis-hum">
					{#each Array(5) as _, i}
						{@const value = metrics.humMin + (i * (metrics.humMax - metrics.humMin)) / 4}
						{@const y = CHART_HEIGHT - CHART_PADDING.bottom - (i * (CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom)) / 4}
						<text x={800 - CHART_PADDING.right + 10} y={y} class="axis-label axis-label-right">
							{value.toFixed(0)}%
						</text>
					{/each}
				</g>
			</svg>

			<div class="chart-footer">
				<span class="time-label">{formatTime(chartData[0].ts)}</span>
				<span class="time-label"
					>{formatTime(chartData[chartData.length - 1].ts)}</span
				>
			</div>
		{/if}
	</div>
</div>

<style>
	.chart-container {
		background: #1a1a1a;
		border: 1px solid #2a2a2a;
		padding: 1.5rem;
		animation: fadeIn 0.8s cubic-bezier(0.4, 0, 0.2, 1) 0.6s both;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid #2a2a2a;
	}

	.chart-title {
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.15em;
		color: #ffffff;
		text-transform: uppercase;
	}

	.chart-legend {
		display: flex;
		gap: 1.5rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.legend-label {
		font-size: 0.65rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		color: #888;
		text-transform: uppercase;
	}

	.chart-wrapper {
		position: relative;
	}

	.chart-empty {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		background: rgba(255, 255, 255, 0.02);
		border: 1px dashed #2a2a2a;
	}

	.empty-text {
		font-size: 0.875rem;
		font-weight: 700;
		letter-spacing: 0.2em;
		color: #444;
		text-transform: uppercase;
	}

	.chart-svg {
		width: 100%;
		height: auto;
		display: block;
	}

	.data-point {
		cursor: pointer;
		transition: r 0.2s ease;
	}

	.data-point:hover {
		r: 5;
	}

	.axis-label {
		font-size: 0.65rem;
		font-family: 'JetBrains Mono', monospace;
		fill: #666;
		dominant-baseline: middle;
	}

	.axis-label-right {
		text-anchor: start;
	}

	.chart-footer {
		display: flex;
		justify-content: space-between;
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid #2a2a2a;
	}

	.time-label {
		font-size: 0.65rem;
		font-family: 'JetBrains Mono', monospace;
		color: #666;
		letter-spacing: 0.05em;
	}

	@media (max-width: 768px) {
		.chart-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.chart-legend {
			flex-direction: column;
			gap: 0.75rem;
		}
	}
</style>
