<script lang="ts">
  import type { Platform } from '../lib/platforms';

  let { platform }: { platform: Platform } = $props();
</script>

<span class="badge" style:--platform-color={platform.color}>
  <span class="dot"></span>
  {platform.name}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    font-size: 13px;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--platform-color);
    box-shadow: 0 0 8px var(--platform-color);
  }
</style>
