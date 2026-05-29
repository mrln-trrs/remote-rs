<script lang="ts">
    import { onMount } from 'svelte';

    // Dinámicamente obtenemos la IP del servidor desde el navegador
    const host = window.location.hostname;
    const API_BASE = `http://${host}:3000`;
    const WS_URL = `ws://${host}:3000/ws`;

    let mediaInfo = $state({
        title: "No media playing",
        artist: "Unknown artist",
        volume: 0,
        is_playing: false,
        thumbnail: null
    });

    let battery = $state({
        percentage: 0,
        is_charging: false
    });

    let isConnected = $state(false);

    function connectWS() {
        const socket = new WebSocket(WS_URL);

        socket.onopen = () => {
            isConnected = true;
            console.log("Connected to Backend");
        };

        socket.onmessage = (event) => {
            const data = JSON.parse(event.data);
            if (data.media) mediaInfo = data.media;
            if (data.battery) battery = data.battery;
        };

        socket.onclose = () => {
            isConnected = false;
            // Reconectar tras 2 segundos
            setTimeout(connectWS, 2000);
        };
    }

    async function sendCommand(command: string) {
        await fetch(`${API_BASE}/api/control/${command}`, { method: 'POST' });
    }

    async function setVolume(level: number) {
        await fetch(`${API_BASE}/api/volume/${level}`, { method: 'POST' });
    }

    onMount(() => {
        connectWS();
    });
</script>

<div class="app-container">
    <header>
        <div class="status-bar">
            <span class="dot {isConnected ? 'online' : 'offline'}"></span>
            <span>{isConnected ? 'Laptop Online' : 'Connecting...'}</span>
            <span class="battery">🔋 {battery.percentage}% {battery.is_charging ? '⚡' : ''}</span>
        </div>
    </header>

    <main>
        <div class="player-card">
            <div class="art-container">
                {#if mediaInfo.thumbnail}
                    <img src="data:image/jpeg;base64,{mediaInfo.thumbnail}" alt="Art" class="album-art" />
                {:else}
                    <div class="art-placeholder">
                        <span class="music-icon">🎵</span>
                    </div>
                {/if}
            </div>

            <div class="track-info">
                <h1>{mediaInfo.title}</h1>
                <h2>{mediaInfo.artist}</h2>
            </div>

            <div class="volume-section">
                <div class="volume-slider-container">
                    <span class="icon">🔈</span>
                    <input 
                        type="range" 
                        min="0" max="100" 
                        value={mediaInfo.volume} 
                        oninput={(e) => setVolume(parseInt(e.currentTarget.value))}
                    />
                    <span class="icon">🔊</span>
                </div>
            </div>

            <div class="controls">
                <button class="secondary" onclick={() => sendCommand('prev')}>⏮</button>
                <button class="play-btn" onclick={() => sendCommand('playpause')}>
                    {mediaInfo.is_playing ? '⏸' : '▶'}
                </button>
                <button class="secondary" onclick={() => sendCommand('next')}>⏭</button>
            </div>
        </div>

        <nav class="quick-actions">
            <button onclick={() => fetch(`${API_BASE}/api/system/lock`, {method: 'POST'})}>🔒 Lock</button>
            <button onclick={() => fetch(`${API_BASE}/api/system/sleep`, {method: 'POST'})}>🌙 Sleep</button>
        </nav>
    </main>
</div>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        background-color: #030303;
        color: white;
        font-family: 'Roboto', sans-serif;
        overflow: hidden;
    }

    .app-container {
        display: flex;
        flex-direction: column;
        height: 100vh;
        width: 100vw;
        max-width: 500px; /* Centrado en pantallas grandes */
        margin: 0 auto;
    }

    header {
        padding: 1rem;
    }

    .status-bar {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.8rem;
        color: #aaa;
    }

    .dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
    }
    .online { background: #0f0; box-shadow: 0 0 5px #0f0; }
    .offline { background: #f00; }
    .battery { margin-left: auto; }

    main {
        flex: 1;
        display: flex;
        flex-direction: column;
        padding: 1rem;
        justify-content: space-around;
    }

    .player-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .art-container {
        width: 80vw;
        height: 80vw;
        max-width: 300px;
        max-height: 300px;
        margin-bottom: 2rem;
        box-shadow: 0 10px 30px rgba(0,0,0,0.5);
        border-radius: 8px;
        overflow: hidden;
        background: #1a1a1a;
    }

    .album-art {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .art-placeholder {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(45deg, #222, #111);
    }

    .music-icon { font-size: 4rem; opacity: 0.3; }

    .track-info h1 {
        font-size: 1.4rem;
        margin: 0 0 0.5rem 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        width: 90vw;
    }

    .track-info h2 {
        font-size: 1.1rem;
        color: #aaa;
        font-weight: normal;
        margin: 0;
    }

    .volume-section {
        width: 100%;
        margin: 2rem 0;
    }

    .volume-slider-container {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 0 1rem;
    }

    input[type="range"] {
        flex: 1;
        accent-color: #ff0000; /* Estilo YT Music */
    }

    .controls {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 2rem;
    }

    button {
        background: transparent;
        border: none;
        color: white;
        cursor: pointer;
        transition: transform 0.1s;
    }

    button:active { transform: scale(0.9); }

    .play-btn {
        background: white;
        color: black;
        width: 64px;
        height: 64px;
        border-radius: 50%;
        font-size: 1.8rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .secondary { font-size: 2rem; }

    .quick-actions {
        display: flex;
        justify-content: center;
        gap: 1rem;
        margin-top: 2rem;
    }

    .quick-actions button {
        background: #222;
        padding: 0.5rem 1rem;
        border-radius: 20px;
        font-size: 0.9rem;
    }
</style>
