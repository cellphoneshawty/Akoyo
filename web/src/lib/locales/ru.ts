import type { MessageKey } from './en';

const ru: Record<MessageKey, string> = {
  'app.tagline': 'Вся твоя музыка в одном месте',

  'offline.title': 'Akoyo не отвечает',
  'offline.text': 'Убедись, что Akoyo запущен на компьютере, и попробуй ещё раз.',
  'offline.retry': 'Попробовать снова',

  'auth.setup.title': 'Добро пожаловать в Akoyo',
  'auth.setup.subtitle': 'Создай первый аккаунт. Он сможет управлять Akoyo для всех домашних.',
  'auth.setup.submit': 'Создать аккаунт',
  'auth.login.title': 'С возвращением',
  'auth.login.subtitle': 'Войди, чтобы слушать свою музыку.',
  'auth.login.submit': 'Войти',
  'auth.login.switch': 'Впервые здесь?',
  'auth.login.switchAction': 'Создать аккаунт',
  'auth.register.title': 'Создай аккаунт',
  'auth.register.subtitle': 'Своя библиотека, свои лайки, свои сервисы.',
  'auth.register.submit': 'Создать аккаунт',
  'auth.register.switch': 'Уже есть аккаунт?',
  'auth.register.switchAction': 'Войти',
  'auth.username': 'Имя пользователя',
  'auth.password': 'Пароль',
  'auth.passwordConfirm': 'Повтори пароль',
  'auth.usernameHint': '3–32 буквы, цифры, точки, дефисы или подчёркивания',
  'auth.passwordHint': 'Минимум 8 символов',

  'nav.search': 'Поиск',
  'nav.liked': 'Любимые треки',
  'nav.settings': 'Настройки',

  'search.placeholder': 'Что хочешь послушать?',
  'search.emptyTitle': 'Ищи по всей своей музыке сразу',
  'search.emptyText': 'Подключи музыкальный сервис в Настройках, и результаты со всех сервисов появятся здесь вместе.',
  'search.connectAction': 'Подключить сервис',

  'liked.title': 'Любимые треки',
  'liked.count': 'Треков: {count}',
  'liked.col.title': 'Название',
  'liked.col.album': 'Альбом',
  'liked.col.platform': 'Платформа',
  'liked.col.added': 'Добавлен',
  'liked.emptyTitle': 'Здесь появятся треки, которые тебе нравятся',
  'liked.emptyText': 'Нажми ❤️ на любом треке из любого сервиса, чтобы сохранить его в библиотеку.',

  'player.idle': 'Сейчас ничего не играет',

  'settings.title': 'Настройки',
  'settings.account': 'Аккаунт',
  'settings.admin': 'Администратор',
  'settings.signOut': 'Выйти',
  'settings.language': 'Язык',
  'settings.services': 'Музыкальные сервисы',
  'settings.servicesText': 'Подключи сервисы, которыми пользуешься. Можно подключить сколько угодно.',
  'settings.connect': 'Подключить',
  'settings.comingSoon': 'Скоро',
  'settings.experimental': 'Экспериментально',
  'settings.offline': 'Музыка офлайн',
  'settings.offlineText': 'Выбери, сколько музыки Akoyo хранит на этом компьютере. Появится в одной из ближайших версий.',
  'settings.about': 'О программе',
  'settings.version': 'Версия {version}',

  'error.invalid_credentials': 'Неверное имя пользователя или пароль.',
  'error.username_taken': 'Это имя пользователя уже занято.',
  'error.invalid_username': 'Имя пользователя: 3–32 буквы, цифры, точки, дефисы или подчёркивания.',
  'error.weak_password': 'Пароль должен быть не короче 8 символов.',
  'error.password_mismatch': 'Пароли не совпадают.',
  'error.not_signed_in': 'Пожалуйста, войди снова.',
  'error.network': 'Не удаётся связаться с Akoyo. Он всё ещё запущен?',
  'error.internal': 'Что-то пошло не так. Попробуй ещё раз.',
};

export default ru;
