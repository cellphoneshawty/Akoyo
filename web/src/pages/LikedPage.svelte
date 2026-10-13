<script lang="ts">
  import EmptyState from '../components/EmptyState.svelte';
  import Icon from '../components/Icon.svelte';
  import { t } from '../lib/i18n.svelte';
</script>

<div class="hero">
  <div class="art"><Icon name="heart" size={64} /></div>
  <div>
    <h1 class="page-title">{t('liked.title')}</h1>
    <p>{t('liked.count', { count: 0 })}</p>
  </div>
</div>

<div class="page">
  <div class="table" role="table">
    <div class="row head" role="row">
      <span role="columnheader">#</span>
      <span role="columnheader">{t('liked.col.title')}</span>
      <span role="columnheader" class="wide">{t('liked.col.album')}</span>
      <span role="columnheader">{t('liked.col.platform')}</span>
      <span role="columnheader" class="wide">{t('liked.col.added')}</span>
      <span role="columnheader" class="duration">⏱</span>
    </div>
  </div>

  <EmptyState icon="heart" title={t('liked.emptyTitle')} text={t('liked.emptyText')} />
</div>

<style>
  .hero {
    display: flex;
    align-items: flex-end;
    gap: 24px;
    padding: 40px clamp(16px, 4vw, 40px) 24px;
    background: linear-gradient(180deg, rgb(139 92 246 / 0.35), transparent);
  }

  .art {
    display: grid;
    flex-shrink: 0;
    place-items: center;
    width: clamp(96px, 16vw, 168px);
    aspect-ratio: 1;
    border-radius: var(--radius);
    background: var(--accent-gradient);
    box-shadow: 0 16px 48px rgb(0 0 0 / 0.45);
    color: #fff;
  }

  .hero p {
    margin: 8px 0 0;
    color: var(--text-muted);
  }

  .page {
    padding-top: 8px;
  }

  .row {
    display: grid;
    grid-template-columns: 40px 3fr 2fr 1.4fr 1.2fr 60px;
    gap: 16px;
    align-items: center;
    padding: 8px 12px;
  }

  .head {
    border-bottom: 1px solid var(--border);
    color: var(--text-faint);
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .duration {
    text-align: right;
  }

  @media (max-width: 720px) {
    .row {
      grid-template-columns: 28px 1fr auto;
    }

    .wide,
    .duration {
      display: none;
    }
  }
</style>
