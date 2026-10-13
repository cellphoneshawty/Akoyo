<script lang="ts">
  let { size = 32, withName = false }: { size?: number; withName?: boolean } = $props();
</script>

<span class="logo">
  <svg width={size} height={size} viewBox="0 0 32 32" aria-hidden="true">
    <defs>
      <linearGradient id="akoyo-logo" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0" stop-color="#8b5cf6" />
        <stop offset="1" stop-color="#ec4899" />
      </linearGradient>
    </defs>
    <circle cx="16" cy="16" r="16" fill="url(#akoyo-logo)" />
    <rect x="9" y="11" width="4" height="10" rx="2" fill="#fff" />
    <rect x="14" y="8" width="4" height="16" rx="2" fill="#fff" />
    <rect x="19" y="10" width="4" height="12" rx="2" fill="#fff" />
  </svg>
  {#if withName}<span class="name">Akoyo</span>{/if}
</span>

<style>
  .logo {
    display: inline-flex;
    align-items: center;
    gap: 10px;
  }

  .name {
    font-size: 20px;
    font-weight: 800;
    letter-spacing: -0.02em;
  }
</style>
