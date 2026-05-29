<script lang="ts">
	import { Pause, Play, Repeat, Repeat1, Shuffle, SkipBack, SkipForward, Volume1, Volume2, VolumeX } from '@lucide/svelte'

	import { media, sendMediaCommand, updateVolume } from './stores/media'

	const transportButtons = [
		{ id: 'prev', icon: SkipBack, label: 'Anterior' },
		{ id: 'playpause', label: 'Reproducir o pausar', isPrimary: true },
		{ id: 'next', icon: SkipForward, label: 'Siguiente' },
	]

	let repeatMode: 'off' | 'all' | 'one' = 'off'
	let shuffleEnabled = false

	function getVolumeIcon(volume: number) {
		if (volume === 0) {
			return VolumeX
		}

		if (volume < 35) {
			return Volume1
		}

		return Volume2
	}

	function toggleRepeat() {
		repeatMode = repeatMode === 'off' ? 'all' : repeatMode === 'all' ? 'one' : 'off'
	}

	function toggleShuffle() {
		shuffleEnabled = !shuffleEnabled
	}

	function handleVolumeInput(event: Event) {
		const input = event.currentTarget as HTMLInputElement | null
		if (input) {
			void updateVolume(Number(input.value))
		}
	}

	function formatTime(ms: number) {
		if (!ms || ms < 0) return '0:00'
		const totalSeconds = Math.floor(ms / 1000)
		const minutes = Math.floor(totalSeconds / 60)
		const seconds = totalSeconds % 60
		return `${minutes}:${seconds.toString().padStart(2, '0')}`
	}
</script>

<section class="controls-card">
	<div class="controls-card__body">
		<!-- Progress bar removed as requested, now handled by PlayerPage immersive view -->

		<div class="transport">
			<button
				type="button"
				class:transport__button--active={shuffleEnabled}
				class="transport__button transport__button--compact"
				onclick={toggleShuffle}
				aria-label="Mezclar"
			>
				<Shuffle size={20} strokeWidth={2.1} />
			</button>

			{#each transportButtons as button}
				<button
					type="button"
					class:transport__button--primary={button.isPrimary}
					class:transport__button--playing={button.isPrimary && $media.isPlaying}
					class="transport__button"
					onclick={() => sendMediaCommand(button.id)}
					aria-label={button.label}
				>
					{#if button.id === 'playpause'}
						{#if $media.isPlaying}
							<Pause size={30} strokeWidth={2.2} />
						{:else}
							<Play size={30} strokeWidth={2.2} />
						{/if}
					{:else}
						<button.icon size={22} strokeWidth={2.2} />
					{/if}
				</button>
			{/each}

			<button
				type="button"
				class:transport__button--active={repeatMode !== 'off'}
				class="transport__button transport__button--compact"
				onclick={toggleRepeat}
				aria-label="Repetición"
			>
				{#if repeatMode === 'one'}
					<Repeat1 size={20} strokeWidth={2.1} />
				{:else}
					<Repeat size={20} strokeWidth={2.1} />
				{/if}
			</button>
		</div>

		<label class="volume">
			<div class="volume__head">
				<span>Volumen</span>
				<strong>{$media.volume}%</strong>
			</div>
			<input
				type="range"
				min="0"
				max="100"
				value={$media.volume}
				aria-label="Control de volumen"
				oninput={handleVolumeInput}
			/>
		</label>
	</div>
</section>

<style>
	.controls-card {
		min-height: 0;
		padding: 0;
		display: grid;
		gap: clamp(10px, 2vw, 14px);
	}

	.controls-card__body {
		display: grid;
		gap: clamp(20px, 4vw, 32px);
		min-height: 0;
	}

	.transport {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: clamp(10px, 2vw, 14px);
		flex-wrap: wrap;
	}

	.transport__button {
		appearance: none;
		border: none;
		border-radius: 0;
		background: rgba(255, 255, 255, 0.03);
		width: clamp(3rem, 12vw, 3.6rem);
		height: clamp(3rem, 12vw, 3.6rem);
		padding: 0;
		display: grid;
		place-items: center;
		justify-items: center;
		cursor: pointer;
		box-shadow: none;
		transition:
			transform 160ms ease,
			background 160ms ease,
			box-shadow 160ms ease;
	}

	.transport__button--compact {
		width: clamp(2.7rem, 11vw, 3.1rem);
		height: clamp(2.7rem, 11vw, 3.1rem);
	}

	.transport__button:hover,
	.transport__button:focus-visible {
		transform: translateY(-1px);
		background: rgba(255, 255, 255, 0.06);
		box-shadow: none;
	}

	.transport__button:focus-visible {
		outline: 2px solid rgba(69, 215, 255, 0.65);
		outline-offset: 2px;
	}

	.transport__button :global(svg) {
		color: var(--text-strong);
	}

	.transport__button--primary {
		width: clamp(4.4rem, 18vw, 5.4rem);
		height: clamp(4.4rem, 18vw, 5.4rem);
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.9);
		box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.12), 0 14px 30px rgba(0, 0, 0, 0.24);
	}

	.transport__button--primary :global(svg) {
		color: black;
	}

	.transport__button--playing {
		box-shadow: 0 0 0 1px rgba(109, 240, 191, 0.15), 0 14px 30px rgba(0, 0, 0, 0.24);
	}

	.transport__button--active {
		border-color: rgba(69, 215, 255, 0.28);
		background: rgba(69, 215, 255, 0.08);
	}

	.volume {
		display: grid;
		gap: 0.5rem;
		padding: 0;
		border-radius: 0;
		border: none;
		background: transparent;
	}

	.volume__head {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 12px;
		color: var(--text-strong);
	}

	.volume input[type='range'] {
		width: 100%;
		appearance: none;
		background: transparent;
		height: 28px;
	}

	.volume input[type='range']::-webkit-slider-runnable-track {
		height: 6px;
		border-radius: 999px;
		background: linear-gradient(90deg, rgba(69, 215, 255, 0.92), rgba(255, 255, 255, 0.12));
	}

	.volume input[type='range']::-webkit-slider-thumb {
		appearance: none;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		border: 2px solid rgba(6, 6, 8, 0.95);
		background: var(--accent-2);
		margin-top: -6px;
		box-shadow: 0 0 0 6px rgba(69, 215, 255, 0.08);
	}

	.volume input[type='range']::-moz-range-track {
		height: 6px;
		border-radius: 999px;
		background: linear-gradient(90deg, rgba(69, 215, 255, 0.92), rgba(255, 255, 255, 0.12));
	}

	.volume input[type='range']::-moz-range-thumb {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(6, 6, 8, 0.95);
		border-radius: 50%;
		background: var(--accent-2);
		box-shadow: 0 0 0 6px rgba(69, 215, 255, 0.08);
	}

	@media (max-width: 599px) {
		.transport {
			gap: 8px;
		}

		.transport__button {
			width: clamp(2.8rem, 14vw, 3.2rem);
			height: clamp(2.8rem, 14vw, 3.2rem);
		}

		.transport__button--primary {
			width: clamp(4rem, 20vw, 4.9rem);
			height: clamp(4rem, 20vw, 4.9rem);
		}
	}
</style>
