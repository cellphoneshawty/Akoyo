<script lang="ts">
  import EmptyState from '../components/EmptyState.svelte';
  import Icon from '../components/Icon.svelte';
  import { t } from '../lib/i18n.svelte';
  import { href } from '../lib/router.svelte';

  let query = $state('');
</script>

<div class="page">
  <form class="search" role="search" onsubmit={(e) => e.preventDefault()}>
    <Icon name="search" />
    <input
      type="search"
      placeholder={t('search.placeholder')}
      aria-label={t('nav.search')}
      bind:value={query}
    />
  </form>

  <EmptyState icon="search" title={t('search.emptyTitle')} text={t('search.emptyText')}>
    <a class="btn btn-primary" href={href('settings')}>{t('search.connectAction')}</a>
  </EmptyState>
</div>

<style>
  .search {
    display: flex;
    align-items: center;
    gap: 12px;
    max-width: 560px;
    padding: 0 18px;
    border: 1px solid var(--border);
    border-radius: 999px;
    background: var(--surface);
    color: var(--text-muted);
    transition: border-color 0.15s;
  }

  .search:focus-within {
    border-color: var(--accent);
  }

  input {
    flex: 1;
    min-width: 0;
    padding: 14px 0;
    border: 0;
    background: none;
    font-size: 16px;
  }

  input:focus {
    outline: none;
  }
</style>
