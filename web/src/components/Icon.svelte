<script lang="ts">
  const paths = {
    search: 'M10.5 3a7.5 7.5 0 0 1 5.96 12.05l4.25 4.24-1.42 1.42-4.24-4.25A7.5 7.5 0 1 1 10.5 3Zm0 2a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11Z',
    heart: 'M12 20.7 4.1 13a5.3 5.3 0 0 1 7.5-7.5l.4.4.4-.4a5.3 5.3 0 0 1 7.5 7.5L12 20.7Z',
    settings:
      'M12 8.5a3.5 3.5 0 1 1 0 7 3.5 3.5 0 0 1 0-7Zm8.6 2.3-1.8-.4a7 7 0 0 0-.7-1.7l1-1.6-1.9-1.9-1.6 1a7 7 0 0 0-1.7-.7L13.5 3h-3l-.4 1.8a7 7 0 0 0-1.7.7l-1.6-1-1.9 1.9 1 1.6a7 7 0 0 0-.7 1.7L3.4 10.8v3l1.8.4c.2.6.4 1.2.7 1.7l-1 1.6 1.9 1.9 1.6-1c.5.3 1.1.5 1.7.7l.4 1.8h3l.4-1.8a7 7 0 0 0 1.7-.7l1.6 1 1.9-1.9-1-1.6c.3-.5.5-1.1.7-1.7l1.8-.4v-3Z',
    play: 'M8 5.5v13l10.5-6.5L8 5.5Z',
    prev: 'M6 6h2v12H6V6Zm3.5 6L18 18V6l-8.5 6Z',
    next: 'M16 6h2v12h-2V6ZM6 18l8.5-6L6 6v12Z',
    music: 'M9 17.5a3 3 0 1 1-2-2.83V5l11-2v12.5a3 3 0 1 1-2-2.83V7.4L9 8.7v8.8Z',
  };

  let { name, size = 22 }: { name: keyof typeof paths; size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d={paths[name]} />
</svg>
