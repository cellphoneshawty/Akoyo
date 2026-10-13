<script lang="ts">
  import { languages, locale } from '../lib/i18n.svelte';

  let { onchange }: { onchange?: (lang: string) => void } = $props();
</script>

<div class="picker" role="group" aria-label="Language">
  {#each languages as language (language.code)}
    <button
      type="button"
      class:active={locale.lang === language.code}
      aria-pressed={locale.lang === language.code}
      onclick={() => {
        locale.set(language.code);
        onchange?.(language.code);
      }}
    >
      {language.name}
    </button>
  {/each}
</div>

<style>
  .picker {
    display: inline-flex;
    padding: 3px;
    border: 1px solid var(--border);
    border-radius: 999px;
    background: var(--bg-elevated);
  }

  button {
    padding: 6px 14px;
    border: 0;
    border-radius: 999px;
    background: none;
    color: var(--text-muted);
    font-size: 13px;
    font-weight: 600;
  }

  button.active {
    background: var(--surface-hover);
    color: var(--text);
  }
</style>
