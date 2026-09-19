# Описание `clap` derive на базе примера

## Что такое `clap` derive

`clap` derive — это **декларативный способ** описания CLI-аргументов. Вместо ручного построения `Command` и `Arg` вы описываете **структуру**, а `clap` **сам генерирует** парсер. Это **короче**, **читаемее** и **типобезопаснее**.

## Разбор примера

### 1. Enum `LogLevel` с `ValueEnum`

```rust
#[derive(Debug, Clone, ValueEnum)]
enum LogLevel {
    Debug,
    Error,
    Warn,
    Trace,
    Info,
}
```

- **`ValueEnum`** — derive-макрос, позволяющий использовать enum как **значение аргумента CLI**.
- Автоматически:
  - преобразует варианты в **kebab-case** (`Debug` → `debug`);
  - добавляет `[possible values: debug, error, warn, trace, info]` в `--help`;
  - парсит значение из строки;
  - выдаёт ошибку при неверном значении.

### 2. Структура `CliArgs` с `Parser`

```rust
#[derive(Parser, Debug)]
struct CliArgs {
    /// Logging threshold
    #[arg(short='l', long = "long-level")]
    log_level: Option<LogLevel>,

    /// The Backend Server Address
    #[arg(short = 'a', long = "server-addres")]
    server_addres: IpAddr,

    /// The Backend Server Port
    #[arg(short = 'p', long = "server-port")]
    server_port: Option<u16>,

    /// Interval between database snapshots in seconds
    #[arg(short = 's', long = "snapshot-freq")]
    snapshot_frequency: Option<usize>
}
```

Разберём:

- **`#[derive(Parser)]`** — генерирует реализацию `Parser`:
  - `CliArgs::parse()` — парсит `std::env::args_os()`;
  - `CliArgs::try_parse()` — возвращает `Result` вместо паники;
  - `CliArgs::command()` — возвращает `Command` для тонкой настройки.

- **Doc-комментарии (`///`)** — становятся **описанием аргумента** в `--help`. Это **идиоматично** для `clap` derive.

- **`#[arg(...)]`** — атрибут для настройки аргумента:
  - `short = 'l'` — короткий флаг `-l`;
  - `long = "long-level"` — длинный флаг `--long-level`.

### 3. Типы полей определяют поведение

| Поле | Тип | Обязательное? | Что парсит |
|---|---|---|---|
| `log_level` | `Option<LogLevel>` | ❌ Нет | `ValueEnum` |
| `server_addres` | `IpAddr` | ✅ Да | IP-адрес |
| `server_port` | `Option<u16>` | ❌ Нет | Число 0–65535 |
| `snapshot_frequency` | `Option<usize>` | ❌ Нет | Число |

**Ключевое правило:**

- **`Option<T>`** — аргумент **необязательный**. Если не передан → `None`.
- **`T`** (без `Option`) — аргумент **обязательный**. Если не передан → ошибка.

Именно поэтому `server_addres: IpAddr` **обязателен** — `clap` требует его указать.

### 4. `main`

```rust
fn main() {
    let args = CliArgs::parse();
    println!("{:?}", args);
    dbg!(args);
}
```

- **`CliArgs::parse()`** — парсит аргументы. Если ошибка или `--help`/`--version` — **завершает программу**.
- **`println!("{:?}", args)`** — вывод через `Debug`.
- **`dbg!(args)`** — макрос для отладки: печатает **файл, строку** и значение.

## Как это работает при запуске

### `--help`

```
$ cargo run -- -h
Usage: c_0012_clap_derive.exe [OPTIONS] --server-addres <SERVER_ADDRES>

Options:
  -l, --long-level <LOG_LEVEL>
          Logging threshold [possible values: debug, error, warn, trace, info]
  -a, --server-addres <SERVER_ADDRES>
          The Backend Server Address
  -p, --server-port <SERVER_PORT>
          The Backend Server Port
  -s, --snapshot-freq <SNAPSHOT_FREQUENCY>
          Interval between database snapshots in seconds
  -h, --help
          Print help
```

Обратите внимание:

- В `USAGE` указано `--server-addres <SERVER_ADDRES>` — **обязательный** аргумент.
- Описания взяты из **doc-комментариев**.
- `[possible values: ...]` — автоматически из `ValueEnum`.

### Полный запуск

```
$ cargo run -- -a 100.10.10.2 -l debug -p 8080 -s 3600
CliArgs { log_level: Some(Debug), server_addres: 100.10.10.2, server_port: Some(8080), snapshot_frequency: Some(3600) }
[src\main.rs:45:5] args = CliArgs {
    log_level: Some(Debug),
    server_addres: 100.10.10.2,
    server_port: Some(8080),
    snapshot_frequency: Some(3600),
}
```

- `-a` → `server_addres: 100.10.10.2` (обязательный).
- `-l debug` → `log_level: Some(Debug)` (ValueEnum).
- `-p 8080` → `server_port: Some(8080)` (u16).
- `-s 3600` → `snapshot_frequency: Some(3600)` (usize).

### Без обязательного аргумента

```
$ cargo run -- -l debug
error: the following required arguments were not provided:
  --server-addres <SERVER_ADDRES>

Usage: c_0012_clap_derive.exe --server-addres <SERVER_ADDRES> [OPTIONS]

For more information, try '--help'.
```

`clap` **сам** сообщает об ошибке и подсказывает `--help`.

### Неверное значение `ValueEnum`

```
$ cargo run -- -a 1.2.3.4 -l invalid
error: invalid value 'invalid' for '--long-level <LOG_LEVEL>'
  [possible values: debug, error, warn, trace, info]
```

## Ключевые атрибуты `#[arg(...)]`

| Атрибут | Назначение |
|---|---|
| `short = 'l'` | Короткий флаг `-l` |
| `long = "long-level"` | Длинный флаг `--long-level` |
| `default_value = "..."` | Значение по умолчанию |
| `default_value_t = ...` | Значение по умолчанию (через `Display`) |
| `value_parser = ...` | Кастомный парсер |
| `value_enum` | Явно указать, что значение — `ValueEnum` |
| `env = "VAR"` | Взять значение из переменной окружения |
| `hide = true` | Скрыть из `--help` |
| `required = true` | Сделать обязательным |
| `num_args = 1..` | Принимать несколько значений |

## Преимущества derive над builder

| | Derive | Builder |
|---|---|---|
| Код | Короче | Длиннее |
| Типобезопасность | ✅ Да | ⚠️ Частично |
| Читаемость | ✅ Высокая | ⚠️ Средняя |
| Гибкость | ⚠️ Ограничена | ✅ Полная |
| Автодополнение | ✅ Да | ✅ Да |

**Рекомендация:** используйте **derive**, если только не нужна **очень тонкая** настройка.

## Типичные ошибки

### 1. Забыть `features = ["derive"]`

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Без этого `#[derive(Parser)]` и `#[derive(ValueEnum)]` **не скомпилируются**.

### 2. Использовать `Option<T>` для обязательного аргумента

```rust
server_addres: Option<IpAddr>,   // ❌ необязательный
server_addres: IpAddr,           // ✅ обязательный
```

### 3. Забыть `value_enum` при использовании enum

В новых версиях `clap` `value_enum` определяется **автоматически** по типу `ValueEnum`. Но иногда нужно указать явно:

```rust
#[arg(value_enum)]
log_level: LogLevel,
```

### 4. Опечатки в `long`

```rust
#[arg(long = "server-addres")]   // ← опечатка: "addres" вместо "address"
```

`clap` не заметит — но пользователи будут вводить **неправильный** флаг.

## Сводная таблица

| Элемент | Назначение |
|---|---|
| `#[derive(Parser)]` | Генерирует парсер для структуры |
| `#[derive(ValueEnum)]` | Позволяет использовать enum как значение |
| `CliArgs::parse()` | Парсит `env::args_os()`, завершает при ошибке |
| `///` | Doc-комментарий → описание в `--help` |
| `#[arg(short, long)]` | Короткий и длинный флаги |
| `Option<T>` | Необязательный аргумент |
| `T` | Обязательный аргумент |
| `IpAddr`, `u16`, `usize` | Автоматический парсинг типа |
| `dbg!(args)` | Отладочный вывод с файлом и строкой |

## Итог

- **`clap` derive** — **декларативный** способ описания CLI.
- **`#[derive(Parser)]`** — генерирует парсер для **структуры**.
- **`#[derive(ValueEnum)]`** — генерирует парсер для **enum**.
- **Doc-комментарии** становятся **описаниями** в `--help`.
- **`Option<T>`** — **необязательный** аргумент; **`T`** — **обязательный**.
- **`clap`** автоматически:
  - парсит типы (`IpAddr`, `u16`, `usize`);
  - показывает `--help`, `--version`;
  - выдаёт ошибки при неверных значениях;
  - поддерживает **kebab-case** для `ValueEnum`.
- **В вашем примере**: `server_addres: IpAddr` — обязательный, остальные — `Option<T>`.
- **Запуск** `cargo run -- -a 100.10.10.2 -l debug -p 8080 -s 3600` парсит все аргументы и выводит структуру `CliArgs`.
