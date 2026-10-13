<script lang="ts">
  import type { Snippet } from 'svelte';
  import Icon from './Icon.svelte';

  let {
    icon,
    title,
    text,
    children,
  }: {
    icon: 'search' | 'heart' | 'music';
    title: string;
    text: string;
    children?: Snippet;
  } = $props();
</script>

<div class="empty">
  <div class="icon"><Icon name={icon} size={30} /></div>
  <h2>{title}</h2>
  <p>{text}</p>
  {@render children?.()}
</div>

<style>
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    max-width: 420px;
    margin: 56px auto;
    text-align: center;
  }

  .icon {
    display: grid;
    place-items: center;
    width: 68px;
    height: 68px;
    border-radius: 50%;
    background: var(--surface);
    color: var(--accent-strong);
  }

  h2 {
    font-size: 20px;
  }

  p {
    margin: 0 0 8px;
    color: var(--text-muted);
  }
</style>
