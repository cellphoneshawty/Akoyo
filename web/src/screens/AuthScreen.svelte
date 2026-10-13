<script lang="ts">
  import LanguagePicker from '../components/LanguagePicker.svelte';
  import Logo from '../components/Logo.svelte';
  import { api, ApiError } from '../lib/api';
  import { errorText, locale, t } from '../lib/i18n.svelte';
  import { session, signedIn } from '../lib/session.svelte';

  type Mode = 'setup' | 'login' | 'register';

  // The starting mode only depends on the first load.
  // svelte-ignore state_referenced_locally
  let mode = $state<Mode>(session.needsSetup ? 'setup' : 'login');
  let username = $state('');
  let password = $state('');
  let passwordConfirm = $state('');
  let error = $state('');
  let busy = $state(false);

  const creating = $derived(mode !== 'login');

  function switchMode(next: Mode) {
    mode = next;
    error = '';
    password = '';
    passwordConfirm = '';
  }

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    error = '';
    if (creating && password !== passwordConfirm) {
      error = errorText('password_mismatch');
      return;
    }
    busy = true;
    try {
      const user = creating
        ? await api.register(username, password, locale.lang)
        : await api.login(username, password);
      signedIn(user);
    } catch (err) {
      error = errorText(err instanceof ApiError ? err.code : 'internal');
    } finally {
      busy = false;
    }
  }
</script>

<div class="screen">
  <header>
    <Logo withName />
    <LanguagePicker />
  </header>

  <main>
    <form class="card" onsubmit={submit}>
      <div class="intro">
        <h1>{t(`auth.${mode}.title`)}</h1>
        <p>{t(`auth.${mode}.subtitle`)}</p>
      </div>

      <div class="field">
        <label for="username">{t('auth.username')}</label>
        <input
          id="username"
          class="input"
          autocomplete="username"
          autocapitalize="none"
          spellcheck="false"
          required
          bind:value={username}
        />
        {#if creating}<span class="hint">{t('auth.usernameHint')}</span>{/if}
      </div>

      <div class="field">
        <label for="password">{t('auth.password')}</label>
        <input
          id="password"
          class="input"
          type="password"
          autocomplete={creating ? 'new-password' : 'current-password'}
          required
          bind:value={password}
        />
        {#if creating}<span class="hint">{t('auth.passwordHint')}</span>{/if}
      </div>

      {#if creating}
        <div class="field">
          <label for="password-confirm">{t('auth.passwordConfirm')}</label>
          <input
            id="password-confirm"
            class="input"
            type="password"
            autocomplete="new-password"
            required
            bind:value={passwordConfirm}
          />
        </div>
      {/if}

      {#if error}<p class="error" role="alert">{error}</p>{/if}

      <button class="btn btn-primary submit" disabled={busy}>
        {t(`auth.${mode}.submit`)}
      </button>

      {#if mode === 'login'}
        <p class="switch">
          {t('auth.login.switch')}
          <a href="#register" onclick={(e) => (e.preventDefault(), switchMode('register'))}>
            {t('auth.login.switchAction')}
          </a>
        </p>
      {:else if mode === 'register'}
        <p class="switch">
          {t('auth.register.switch')}
          <a href="#login" onclick={(e) => (e.preventDefault(), switchMode('login'))}>
            {t('auth.register.switchAction')}
          </a>
        </p>
      {/if}
    </form>
  </main>
</div>

<style>
  .screen {
    display: flex;
    flex-direction: column;
    min-height: 100%;
    background:
      radial-gradient(60rem 30rem at 15% -10%, rgb(139 92 246 / 0.22), transparent 60%),
      radial-gradient(50rem 30rem at 110% 110%, rgb(236 72 153 / 0.16), transparent 60%),
      var(--bg);
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 20px clamp(16px, 4vw, 40px);
  }

  main {
    display: grid;
    flex: 1;
    place-items: center;
    padding: 16px 16px 64px;
  }

  .card {
    display: flex;
    flex-direction: column;
    gap: 18px;
    width: 100%;
    max-width: 400px;
    padding: 32px;
    border: 1px solid var(--border);
    border-radius: 20px;
    background: rgb(21 20 29 / 0.85);
    backdrop-filter: blur(12px);
    box-shadow: 0 24px 64px rgb(0 0 0 / 0.4);
  }

  .intro h1 {
    font-size: 26px;
    letter-spacing: -0.02em;
  }

  .intro p {
    margin: 8px 0 0;
    color: var(--text-muted);
  }

  .error {
    margin: 0;
    padding: 10px 14px;
    border-radius: var(--radius-sm);
    background: rgb(248 113 113 / 0.12);
    color: var(--danger);
    font-size: 14px;
  }

  .submit {
    margin-top: 4px;
    padding: 12px;
  }

  .switch {
    margin: 0;
    text-align: center;
    color: var(--text-muted);
    font-size: 14px;
  }

  @media (max-width: 480px) {
    .card {
      padding: 24px 20px;
    }
  }
</style>
