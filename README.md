# rust_lab

Учебный workspace на Rust для экспериментов в нескольких направлениях: CLI, графика, backend и небольшие sandbox-примеры.

## Структура проекта

```text
rust_lab/
├── Cargo.toml                 # Workspace
├── apps/
│   ├── inquire_app/           # CLI с валидацией email/пароля
│   ├── raylib_app/            # 2D-симуляция на raylib
│   ├── server_app/            # Axum + SQLx (SQLite)
│   └── test_samples/          # Короткие regex-примеры
└── architecture_recommendations.md
```

## Требования

- Rust toolchain (рекомендуется stable)
- Cargo
- Для `raylib_app` нужны системные зависимости `raylib` (зависит от ОС)

## Быстрый старт

Из корня проекта:

```bash
cargo build --workspace
```

Запуск приложений:

```bash
cargo run -p inquire_app
cargo run -p raylib_app
cargo run -p server_app
cargo run -p test_samples
```

## Проверки качества

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## server_app: конфигурация

`server_app` читает настройки из переменных окружения (`.env` поддерживается через `dotenvy`).

Шаблон конфигурации:

- `apps/server_app/.env.example`

Минимально необходимые/полезные переменные:

- `DATABASE_URL` (обязательная), пример: `sqlite:./server_app.sqlite?mode=rwc`
- `HOST` (опционально, default `0.0.0.0`)
- `PORT` (опционально, default `8080`)

## Для экспериментов

- Добавляй новые мини-проекты в `apps/` как отдельные crates.
- Если эксперимент одноразовый, держи его изолированно (по аналогии с `test_samples`).
- Старайся держать один фокус на crate: так проще сравнивать подходы и мерить прогресс.
