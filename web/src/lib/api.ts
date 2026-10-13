export type User = {
  id: number;
  username: string;
  is_admin: boolean;
  language: string;
};

export type Status = {
  app: string;
  version: string;
  needs_setup: boolean;
};

export class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
  ) {
    super(code);
  }
}

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
  let res: Response;
  try {
    res = await fetch(`/api${path}`, {
      method,
      headers: body === undefined ? undefined : { 'Content-Type': 'application/json' },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
  } catch {
    throw new ApiError(0, 'network');
  }

  if (!res.ok) {
    let code = 'internal';
    try {
      code = (await res.json()).error ?? code;
    } catch {
      // not a JSON error body
    }
    throw new ApiError(res.status, code);
  }
  if (res.status === 204) return undefined as T;
  return res.json();
}

export const api = {
  status: () => request<Status>('GET', '/status'),
  register: (username: string, password: string, language: string) =>
    request<User>('POST', '/auth/register', { username, password, language }),
  login: (username: string, password: string) =>
    request<User>('POST', '/auth/login', { username, password }),
  logout: () => request<void>('POST', '/auth/logout'),
  me: () => request<User>('GET', '/me'),
  updateMe: (changes: { language?: string }) => request<User>('PATCH', '/me', changes),
};
