<script lang="ts">
  import PageHeader from '../PageHeader.svelte'
  import { sendSystemAction } from '../api/backend'
</script>

<section class="page-frame">
  <PageHeader
    eyebrow="Sistema"
    title="Estado"
    text="Pantalla para enlaces, dispositivo activo y telemetría, también con botón de volver arriba."
  />

  <div class="system-grid">
    <article class="system-card">
      <span>Wi-Fi</span>
      <strong>Listo para enlazar</strong>
    </article>

    <article class="system-card">
      <span>Dispositivo</span>
      <strong>Laptop principal</strong>
    </article>

    <article class="system-card">
      <span>Modo</span>
      <strong>Fluent blur</strong>
    </article>
  </div>

  <div class="system-actions">
    <button type="button" class="system-action" onclick={() => sendSystemAction('lock')}>Bloquear</button>
    <button type="button" class="system-action" onclick={() => sendSystemAction('sleep')}>Suspender</button>
    <button type="button" class="system-action" onclick={() => sendSystemAction('reboot')}>Reiniciar</button>
    <button type="button" class="system-action" onclick={() => sendSystemAction('shutdown')}>Apagar</button>
  </div>
</section>

<style>
  .page-frame {
    height: 100%;
    min-height: 0;
    display: grid;
    grid-template-rows: auto auto auto;
    gap: 12px;
  }

  .system-grid {
    display: grid;
    gap: 10px;
  }

  .system-actions {
    display: grid;
    gap: 10px;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .system-card {
    padding: 18px;
    border-radius: 0;
    border: 1px solid var(--border);
    background: transparent;
    display: grid;
    gap: 4px;
  }

  .system-card span {
    color: var(--muted);
    font-size: 0.82rem;
    text-transform: uppercase;
    letter-spacing: 0.18em;
  }

  .system-card strong {
    color: var(--text-strong);
    font-size: 1rem;
  }

  .system-action {
    appearance: none;
    border: 1px solid var(--border);
    border-radius: 0;
    background: transparent;
    color: var(--text-strong);
    padding: 14px;
    cursor: pointer;
  }

  .system-action:hover,
  .system-action:focus-visible {
    border-color: rgba(255, 255, 255, 0.22);
    outline: none;
  }

  @media (min-width: 860px) {
    .system-grid {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }

    .system-actions {
      grid-template-columns: repeat(4, minmax(0, 1fr));
    }
  }
</style>