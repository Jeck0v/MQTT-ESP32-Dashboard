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
	const CHART_PADDING = { top: 20, right: 20, bottom: 60, left: 50 };
	const MAX_BARS = 20;

	// Get unique devices
	let devices = $derived(
		Array.from(new Set(chartData.map((d) => d.deviceId))).sort()
	);

	let timestamps = $derived(() => {
		const uniqueTs = Array.from(new Set(chartData.map((d) => d.ts)))
			.sort((a, b) => a - b)
			.slice(-MAX_BARS);
		return uniqueTs;
	});

	let groupedData = $derived(() => {
		const grouped = new Map();
		timestamps().forEach((ts) => {
			const deviceData = new Map();
			devices.forEach((deviceId) => {
				const point = chartData.find((d) => d.ts === ts && d.deviceId === deviceId);
				if (point) {
					deviceData.set(deviceId, point);
				}
			});
			grouped.set(ts, deviceData);
		});
		return grouped;
	});

	let tempMetrics = $derived({
		min: chartData.length > 0 ? Math.floor(Math.min(...chartData.map((d) => d.tempC)) - 2) : 0,
		max: chartData.length > 0 ? Math.ceil(Math.max(...chartData.map((d) => d.tempC)) + 2) : 30
	});

	let humMetrics = $derived({
		min: chartData.length > 0 ? Math.floor(Math.min(...chartData.map((d) => d.humPct)) - 5) : 0,
		max: chartData.length > 0 ? Math.ceil(Math.max(...chartData.map((d) => d.humPct)) + 5) : 100
	});

	/**
	 * Scale value to chart coordinates
	 * @param {number} value
	 * @param {number} min
	 * @param {number} max
	 * @returns {number}
	 */
	function scaleY(value, min, max) {
		const range = max - min || 1;
		const chartHeight = CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom;
		return ((value - min) / range) * chartHeight;
	}

	/**
	 * Get X position for timestamp group
	 * @param {number} index
	 * @param {number} width
	 * @returns {number}
	 */
	function getGroupX(index, width) {
		const chartWidth = width - CHART_PADDING.left - CHART_PADDING.right;
		const groupWidth = chartWidth / timestamps().length;
		return CHART_PADDING.left + index * groupWidth;
	}

	/**
	 * Get bar width for each device
	 * @param {number} width
	 * @returns {number}
	 */
	function getBarWidth(width) {
		const chartWidth = width - CHART_PADDING.left - CHART_PADDING.right;
		const groupWidth = chartWidth / timestamps().length;
		const deviceCount = devices.length;
		const barWidth = (groupWidth * 0.8) / deviceCount; // 80% of group width for bars
		return Math.max(barWidth, 2); // Minimum 2px width
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
	 * Format timestamp for axis label (short version)
	 * @param {number} ts
	 * @returns {string}
	 */
	function formatTimeShort(ts) {
		const date = new Date(ts * 1000);
		return date.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
	}

	/**
	 * Get device color
	 * @param {string} deviceId
	 * @returns {string}
	 */
	function getDeviceColor(deviceId) {
		const colors = [
			'#00ffff', // Cyan
			'#ff00ff', // Magenta
			'#ffff00', // Yellow
			'#00ff80', // Green
			'#ff8800', // Orange
			'#00aaff', // Blue
			'#ff0080', // Pink
			'#80ff00', // Lime
			'#ff0000', // Red
			'#8000ff'  // Purple
		];
		const index = devices.indexOf(deviceId);
		return colors[index % colors.length];
	}
</script>

<div class="charts-container">
	<!-- Temperature Bar Chart -->
	<div class="chart-container">
		<div class="chart-header">
			<div class="chart-title">TEMPERATURE (°C)</div>
			<div class="chart-legend">
				{#each devices as deviceId}
					<div class="legend-item">
						<span class="legend-dot" style="background: {getDeviceColor(deviceId)};"></span>
						<span class="legend-label">{deviceId}</span>
					</div>
				{/each}
			</div>
		</div>

		<div class="chart-wrapper">
			{#if chartData.length === 0}
				<div class="chart-empty">
					<div class="empty-text">NO DATA AVAILABLE</div>
				</div>
			{:else}
				<svg class="chart-svg" viewBox="0 0 800 {CHART_HEIGHT}">
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

					<!-- Y-axis labels -->
					<g class="y-axis">
						{#each Array(5) as _, i}
							{@const value = tempMetrics.min + (i * (tempMetrics.max - tempMetrics.min)) / 4}
							{@const y = CHART_HEIGHT - CHART_PADDING.bottom - (i * (CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom)) / 4}
							<text x={CHART_PADDING.left - 10} y={y} class="axis-label" text-anchor="end">
								{value.toFixed(1)}
							</text>
						{/each}
					</g>

					<!-- Temperature bars -->
					<g class="bars">
						{#each timestamps() as ts, tsIndex}
							{@const deviceData = groupedData().get(ts)}
							{@const groupX = getGroupX(tsIndex, 800)}
							{@const barWidth = getBarWidth(800)}
							{#if deviceData}
								{#each devices as deviceId, deviceIndex}
									{@const point = deviceData.get(deviceId)}
									{#if point}
										{@const color = getDeviceColor(deviceId)}
										{@const barHeight = scaleY(point.tempC, tempMetrics.min, tempMetrics.max)}
										{@const barX = groupX + deviceIndex * barWidth + (0.1 * (groupX + barWidth * devices.length - groupX)) / 2}
										{@const barY = CHART_HEIGHT - CHART_PADDING.bottom - barHeight}
										<rect
											x={barX}
											y={barY}
											width={barWidth}
											height={barHeight}
											fill={color}
											opacity="0.8"
											class="bar"
										>
											<title>{deviceId}: {point.tempC.toFixed(1)}°C at {formatTime(point.ts)}</title>
										</rect>
									{/if}
								{/each}
							{/if}
						{/each}
					</g>

					<!-- X-axis labels -->
					<g class="x-axis">
						{#each timestamps() as ts, index}
							{@const groupX = getGroupX(index, 800)}
							{@const groupWidth = (800 - CHART_PADDING.left - CHART_PADDING.right) / timestamps().length}
							<text
								x={groupX + groupWidth / 2}
								y={CHART_HEIGHT - CHART_PADDING.bottom + 20}
								class="axis-label x-label"
								text-anchor="middle"
								transform="rotate(-45, {groupX + groupWidth / 2}, {CHART_HEIGHT - CHART_PADDING.bottom + 20})"
							>
								{formatTimeShort(ts)}
							</text>
						{/each}
					</g>
				</svg>
			{/if}
		</div>
	</div>

	<!-- Humidity Bar Chart -->
	<div class="chart-container">
		<div class="chart-header">
			<div class="chart-title">HUMIDITY (%)</div>
			<div class="chart-legend">
				{#each devices as deviceId}
					<div class="legend-item">
						<span class="legend-dot" style="background: {getDeviceColor(deviceId)};"></span>
						<span class="legend-label">{deviceId}</span>
					</div>
				{/each}
			</div>
		</div>

		<div class="chart-wrapper">
			{#if chartData.length === 0}
				<div class="chart-empty">
					<div class="empty-text">NO DATA AVAILABLE</div>
				</div>
			{:else}
				<svg class="chart-svg" viewBox="0 0 800 {CHART_HEIGHT}">
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

					<!-- Y-axis labels -->
					<g class="y-axis">
						{#each Array(5) as _, i}
							{@const value = humMetrics.min + (i * (humMetrics.max - humMetrics.min)) / 4}
							{@const y = CHART_HEIGHT - CHART_PADDING.bottom - (i * (CHART_HEIGHT - CHART_PADDING.top - CHART_PADDING.bottom)) / 4}
							<text x={CHART_PADDING.left - 10} y={y} class="axis-label" text-anchor="end">
								{value.toFixed(0)}
							</text>
						{/each}
					</g>

					<!-- Humidity bars -->
					<g class="bars">
						{#each timestamps() as ts, tsIndex}
							{@const deviceData = groupedData().get(ts)}
							{@const groupX = getGroupX(tsIndex, 800)}
							{@const barWidth = getBarWidth(800)}
							{#if deviceData}
								{#each devices as deviceId, deviceIndex}
									{@const point = deviceData.get(deviceId)}
									{#if point}
										{@const color = getDeviceColor(deviceId)}
										{@const barHeight = scaleY(point.humPct, humMetrics.min, humMetrics.max)}
										{@const barX = groupX + deviceIndex * barWidth + (0.1 * (groupX + barWidth * devices.length - groupX)) / 2}
										{@const barY = CHART_HEIGHT - CHART_PADDING.bottom - barHeight}
										<rect
											x={barX}
											y={barY}
											width={barWidth}
											height={barHeight}
											fill={color}
											opacity="0.8"
											class="bar"
										>
											<title>{deviceId}: {point.humPct.toFixed(1)}% at {formatTime(point.ts)}</title>
										</rect>
									{/if}
								{/each}
							{/if}
						{/each}
					</g>

					<!-- X-axis labels -->
					<g class="x-axis">
						{#each timestamps() as ts, index}
							{@const groupX = getGroupX(index, 800)}
							{@const groupWidth = (800 - CHART_PADDING.left - CHART_PADDING.right) / timestamps().length}
							<text
								x={groupX + groupWidth / 2}
								y={CHART_HEIGHT - CHART_PADDING.bottom + 20}
								class="axis-label x-label"
								text-anchor="middle"
								transform="rotate(-45, {groupX + groupWidth / 2}, {CHART_HEIGHT - CHART_PADDING.bottom + 20})"
							>
								{formatTimeShort(ts)}
							</text>
						{/each}
					</g>
				</svg>
			{/if}
		</div>
	</div>
</div>

<style>
	.charts-container {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

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
		flex-direction: column;
		gap: 1rem;
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
		flex-wrap: wrap;
		gap: 1rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 2px;
		box-shadow: 0 0 8px currentColor;
	}

	.legend-label {
		font-size: 0.65rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		color: #888;
		text-transform: uppercase;
		font-family: 'JetBrains Mono', monospace;
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

	.bar {
		cursor: pointer;
		transition: opacity 0.2s ease;
	}

	.bar:hover {
		opacity: 1 !important;
		filter: brightness(1.2);
	}

	.axis-label {
		font-size: 0.65rem;
		font-family: 'JetBrains Mono', monospace;
		fill: #666;
		dominant-baseline: middle;
	}

	.x-label {
		font-size: 0.6rem;
	}

	@media (max-width: 1200px) {
		.charts-container {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.chart-header {
			gap: 0.75rem;
		}

		.chart-legend {
			gap: 0.75rem;
		}

		.x-label {
			font-size: 0.55rem;
		}
	}
</style>
