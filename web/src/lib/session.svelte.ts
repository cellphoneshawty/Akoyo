import { api, ApiError, type User } from './api';
import { locale } from './i18n.svelte';

export const session = $state({
  ready: false,
  offline: false,
  needsSetup: false,
  version: '',
  user: null as User | null,
});

export async function bootstrap() {
  session.offline = false;
  try {
    const status = await api.status();
    session.needsSetup = status.needs_setup;
    session.version = status.version;
    if (!status.needs_setup) {
      try {
        signedIn(await api.me());
      } catch (err) {
        if (!(err instanceof ApiError && err.status === 401)) throw err;
      }
    }
  } catch {
    session.offline = true;
  } finally {
    session.ready = true;
  }
}

export function signedIn(user: User) {
  session.user = user;
  session.needsSetup = false;
  locale.set(user.language);
}

export async function signOut() {
  await api.logout().catch(() => {});
  session.user = null;
}
