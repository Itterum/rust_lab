# Рекомендации по архитектуре проекта rust_lab

**Дата:** 2026-05-03  
**Статус:** Обсуждение (без изменений кода)

---

## Текущее состояние

### Структура workspace
```
rust_lab/
├── Cargo.toml (workspace)
├── Cargo.lock
├── apps/
│   ├── inquire_app/     # CLI приложение с вводом данных
│   ├── raylib_app/      # 2D симуляция солнечной системы
│   ├── server_app/      # Axum сервер с SQLite
│   └── test_samples/    # Примеры использования regex
└── target/
```

### Текущие приложения

| Приложение | Депы | Описание |
|------------|------|----------|
| `inquire_app` | `inquire`, `regex` | Валидация email и пароля |
| `raylib_app` | `raylib` | Графическая симуляция орбит |
| `server_app` | `axum`, `sqlx` (SQLite) | REST API endpoint |
| `test_samples` | `regex` | Простой пример |

---

## Общие проблемы

### 1. Отсутствие документации
- Нет `README.md` в корне проекта
- Нет описания у каждого app
- Нет инструкций по запуску

### 2. Жестко заданный путь в `server_app`
```rust
// apps/server_app/src/main.rs:54-58
let pool = SqlitePool::connect(
    "sqlite:C:\\Users\\ivanl\\DataGripProjects\\database_app\\identifier.sqlite?mode=rwc",
).await.unwrap();
```
- Привязан к конкретному Windows-пути
- Непереносим между ОС и машинами
- Нет конфигурации через `.env` или аргументы

### 3. Отсутствие инфраструктуры для разработки
- Нет Docker-конфигурации
- Нет `Makefile`/скриптов для automation

---

## Рекомендуемые улучшения

### A. Docker Compose для инфраструктуры

Создать `docker-compose.yml` в корне:

```yaml
version: '3.8'
services:
  db:
    image: postgres:16-alpine
    container_name: rust_lab_db
    ports:
      - "5432:5432"
    environment:
      POSTGRES_DB: rust_lab
      POSTGRES_USER: rust_user
      POSTGRES_PASSWORD: rust_pass
    volumes:
      - rust_lab_data:/var/lib/postgresql/data

  adminer:
    image: adminer:latest
    ports:
      - "8080:8080"
    depends_on:
      - db

volumes:
  rust_lab_data:
```

**Плюсы:**
- Единая среда разработки
- Быстрый старт через `docker-compose up -d`
- Можно поднимать разные БД (Postgres, MySQL, SQLite через volume)

### B. Улучшение `server_app`

#### 1. Вынести конфигурацию
```
apps/server_app/src/
├── main.rs       # Точка входа, parse args/env
├── config.rs     # Config struct + env parser
├── db.rs         # Pool initialization, migrations
└── api/
    ├── mod.rs
    └── routes/
        └── user.rs
```

#### 2. Поддержка нескольких БД
```
[dependencies]
sqlx = { version = "0.8", features = ["postgres", "sqlite", "runtime-tokio"] }

[features]
default = ["postgres"]
postgres = []
sqlite = []
```

#### 3. Миграции базы данных
- `embark-studios/sea-orm` — альтернатива sqlx с встроенными миграциями
- `diesel_migrations` — если использовать Diesel
- `sqlx-cli` — миграции через CLI инструмент

### C. Общие модули (apps/common)

```rust
// apps/common/Cargo.toml
[package]
name = "common"
version = "0.1.0"

[dependencies]
thiserror = "1.0"
tracing = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Что можно вынести:**
- `error.rs` — общие error types
- `logger.rs` — логирование через `tracing`
- `types.rs` — общие DTO/Types
- `config.rs` — универсальный config loader

### D. Дополнительные идеи

#### 1. `apps/experiments/`
Папка "для игр" — код без build cache, без строгой структуры:

```
apps/experiments/
├── 01_websocket_chat/
├── 02_graphql_server/
├── 03_grpc_service/
└── 04_wasm_module/
```

#### 2. Makefile для automation

```makefile
.PHONY: up down run-server run-cli run-ray test lint

up:
	docker compose up -d db

down:
	docker compose down

run-server:
	cargo run -p server_app

run-cli:
	cargo run -p inquire_app

run-ray:
	cargo run -p raylib_app

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -W clippy::all
```

#### 3. CI/CD скелет
`.github/workflows/ci.yml`:
- Карт: `cargo build --workspace`
- Тесты: `cargo test --workspace`
- Линт: `cargo clippy --workspace`

#### 4. Rust Toolchain
Создать `rust-toolchain.toml`:
```toml
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
```

---

## Детальный анализ каждого приложения

### `inquire_app`

#### Что хорошо:
- Пример валидации email через regex
- Пример валидации пароля (8+ символов, буквы+цифры)
- Использование `inquire` — правильный выбор для CLI

#### Рекомендации:
1. Вынести валидаторы в `common` для переиспользования
2. Добавить unit-тесты валидаторов
3. Возможность читать input из stdin
4. Возможность записать output в файл

### `raylib_app`

#### Что хорошо:
- Объектно-ориентированный подход (структура `CelestialBody`)
- Рекурсивная отрисовка спутников
- Использование `dt` (delta time) для кадровой независимости

#### Рекомендации:
1. Вынести рендеринг в отдельный модуль
2. Добавить управление камерой (zoom, пан)
3. Добавить конфигурацию через JSON/YAML
4. Возможность экспорта анимации (GIF/WebM)

### `server_app`

#### Что хорошо:
- Правильное использование `State` для pool
- Использование `sqlx::FromRow`
- Простая REST endpoint структура

#### Критические проблемы:
1. **Жестко заданный путь** — см. выше
2. Отсутствие middleware для logging/cors
3. Нет обработки errors через `Json` (возвращает статус, но без body)
4. Нет unit/integration тестов

#### Рекомендации:
1. Использовать `.env`+`dotenvy` для конфигурации
2. Вынести routes в отдельные модули
3. Добавить `tower_http` middleware
4. Добавить OpenAPI/Swagger через `axum-extras`
5. Добавить integration тесты через `test_case` или `mockall`

### `test_samples`

#### Что хорошо:
- Простой и понятный пример

#### Рекомендации:
1. Перенести в `apps/experiments/`
2. Добавить больше примеров (проверка IP, URL, phone numbers)
3. Добавить unit-тесты

---

## План реализации (при согласии)

| Этап | Задача | Приоритет |
|------|--------|-----------|
| 1 | Создать `docker-compose.yml` | Высокий |
| 2 | Создать `Makefile` с базовыми командами | Средний |
| 3 | Вынести конфиг в `server_app/src/config.rs` | Высокий |
| 4 | Создать `apps/common` | Средний |
| 5 | Добавить `README.md` для каждого app | Низкий |
| 6 | Добавить unit-тесты | Низкий |
| 7 | Добавить CI/CD скелет | Низкий |

---

## Дополнительные вопросы для обсуждения

1. **Какую БД предпочитаешь для разработки?**
   - PostgreSQL (рекомендуется для production)
   - SQLite (просто для dev)
   - MySQL
   - Другое

2. **Хочешь ли интегрировать CI/CD?**
   - GitHub Actions (по умолчанию)
   - GitLab CI
   - Другое

3. **Что добавить в `apps/experiments/` первым?**
   - WebSocket чат
   - GraphQL сервер
   - gRPC сервис
   - WASM module

4. **Планируешь ли ты делать приложение dockerized в production?**
   - Да, `docker build` + `docker-compose`
   - Да, но с Helm/Kubernetes
   - Нет, только dev/test
   - Не определился

---

*Файл обновляется при добавлении новых рекомендаций*
