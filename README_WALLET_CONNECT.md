# 🔗 WalletConnect API для Qubic - Реализация на Rust

Полнофункциональная библиотека на Rust для подключения dApp к Qubic кошелькам через WalletConnect v2 с поддержкой QR-кодов.

## ✨ Особенности

- ✅ **Полная реализация WalletConnect v2** - совместимость с официальной спецификацией
- ✅ **Qubic Namespace** - поддержка всех методов Qubic блокчейна
- ✅ **WASM Support** - компиляция для использования в браузере
- ✅ **Type-Safe** - строгая типизация на уровне компилятора
- ✅ **Async/Await** - современная асинхронная архитектура
- ✅ **Event System** - гибкая система обработки событий
- ✅ **QR Code Integration** - генерация URI для QR-кодов
- ✅ **Session Management** - автоматическое управление сессиями
- ✅ **Error Handling** - надежная обработка ошибок

## 📦 Установка

### Rust проект

```toml
[dependencies]
scapi = { path = "path/to/scapi" }
tokio = { version = "1", features = ["full"] }
```

### WASM для браузера

```bash
# Установка wasm-pack
cargo install wasm-pack

# Сборка
cd path/to/scapi
wasm-pack build --target web --out-dir pkg
```

## 🚀 Быстрый старт

### Rust (Native)

```rust
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Создание конфигурации
    let config = WalletConnectConfig::new(
        "your_project_id".to_string(),
        "qubic:mainnet".to_string()
    );

    // 2. Инициализация клиента
    let mut client = WalletConnectClient::new(config);
    client.init().await?;

    // 3. Генерация QR-кода
    let uri = client.connect().await?;
    println!("Scan QR: {}", uri);

    // 4. После подключения - запрос аккаунтов
    let accounts = client.request_accounts().await?;
    println!("Accounts: {:?}", accounts);

    // 5. Отправка транзакции
    let params = QubicTransactionParams {
        from: accounts[0].address.clone(),
        to: "RECIPIENT_ADDRESS".to_string(),
        amount: 1_000_000,
        tick: None,
        input_type: Some(0),
        payload: None,
    };

    let result = client.send_transaction(params).await?;
    println!("Result: {:?}", result);

    Ok(())
}
```

### JavaScript/TypeScript (WASM)

```javascript
import init, { WalletConnectClient } from './pkg/scapi.js';
import QRCode from 'qrcode';

async function connectWallet() {
    // Инициализация WASM
    await init();

    // Создание клиента
    const client = new WalletConnectClient(
        'your_project_id',
        'qubic:mainnet'
    );

    // Инициализация
    await client.initClient();

    // Генерация URI
    const uri = await client.genConnectUrl();
    
    // Отображение QR-кода
    await QRCode.toCanvas(document.getElementById('qrcode'), uri);

    // Запрос аккаунтов
    const accounts = await client.requestAccounts();
    console.log('Connected:', accounts);

    // Отправка транзакции
    const result = await client.sendTransaction({
        from: accounts[0].address,
        to: 'RECIPIENT_ADDRESS',
        amount: 1000000,
        inputType: 0,
        payload: null
    });

    console.log('Transaction:', result);
}
```

## 📚 Документация

- **[Полная документация API](./WALLET_CONNECT_API.md)** - детальное описание всех методов
- **[Быстрый старт](./WALLET_CONNECT_QUICKSTART.md)** - 5-минутная интеграция
- **[Примеры использования](./examples/)** - готовые примеры кода
- **[README модуля](./src/wallet_connect/README.md)** - внутренняя архитектура

## 🎯 Основные возможности

### 1. Подключение к кошельку

```rust
let uri = client.connect().await?;
// Отобразите uri как QR-код для сканирования
```

### 2. Управление аккаунтами

```rust
let accounts = client.request_accounts().await?;
for account in accounts {
    println!("Address: {}", account.address);
    if let Some(balance) = account.amount {
        println!("Balance: {}", balance);
    }
}
```

### 3. Транзакции

```rust
// Простая отправка
client.send_qubic("from", "to", 1000000).await?;

// Сложная транзакция
let params = QubicTransactionParams {
    from: "SENDER".to_string(),
    to: "RECIPIENT".to_string(),
    amount: 1_000_000,
    tick: Some(12345),
    input_type: Some(1),
    payload: Some("custom_data".to_string()),
};
client.send_transaction(params).await?;
```

### 4. Подписание

```rust
// Подписание транзакции (без отправки)
let signature = client.sign_transaction(params).await?;

// Подписание сообщения
let signature = client.sign_message("from_address", "message").await?;
```

### 5. Обработка событий

```rust
use scapi::wallet_connect::events::*;

struct MyHandler;

impl EventCallback for MyHandler {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        match event {
            WalletConnectEvent::SessionDelete => println!("Disconnected!"),
            WalletConnectEvent::SessionExpire => println!("Expired!"),
            _ => println!("Event: {:?}", event),
        }
    }
}

client.event_handler().register(Box::new(MyHandler));
```

## 📁 Структура проекта

```
src/wallet_connect/
├── mod.rs              # Основной модуль
├── types.rs            # Типы и структуры данных
├── client.rs           # WalletConnect клиент
├── session.rs          # Управление сессиями
├── events.rs           # Система событий
├── qubic_namespace.rs  # Qubic методы
└── wasm_bindings.rs    # WASM биндинги
```

## 🧪 Примеры

Проект включает 3 полных примера:

```bash
# 1. Базовое подключение
cargo run --example wallet_connect_basic

# 2. Отправка транзакции
cargo run --example wallet_connect_transaction

# 3. Обработка событий
cargo run --example wallet_connect_events
```

## 🔧 API Reference

### WalletConnectClient

| Метод | Описание |
|-------|----------|
| `new(config)` | Создание клиента |
| `init()` | Инициализация |
| `connect()` | Генерация URI для QR |
| `request_accounts()` | Запрос аккаунтов |
| `send_qubic(from, to, amount)` | Простая отправка |
| `sign_transaction(params)` | Подписание транзакции |
| `send_transaction(params)` | Отправка транзакции |
| `sign_message(from, msg)` | Подписание сообщения |
| `disconnect()` | Отключение |
| `is_session_active()` | Проверка активности |

### Типы данных

```rust
// Конфигурация
pub struct WalletConnectConfig {
    pub project_id: String,
    pub qubic_chain_id: String,
    pub metadata: ClientMetadata,
    pub relay_url: Option<String>,
}

// Аккаунт
pub struct WalletAccount {
    pub address: String,
    pub amount: Option<u64>,
    pub additional_data: HashMap<String, serde_json::Value>,
}

// Параметры транзакции
pub struct QubicTransactionParams {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub tick: Option<u64>,
    pub input_type: Option<u16>,
    pub payload: Option<String>,
}
```

## 🌐 Интеграция с существующим TypeScript кодом

### Миграция с TypeScript

**До (TypeScript):**
```typescript
import WalletConnectClient from './api/wallet-connect-client';

const client = new WalletConnectClient();
await client.initClient();
const { uri } = await client.genConnectUrl();
```

**После (Rust WASM):**
```javascript
import { WalletConnectClient } from './pkg/scapi.js';

const client = new WalletConnectClient('project_id', 'qubic:mainnet');
await client.initClient();
const uri = await client.genConnectUrl();
```

## 🔒 Безопасность

- ✅ Все криптографические операции в кошельке
- ✅ Приватные ключи не передаются
- ✅ Защищенное соединение через relay
- ✅ Проверка подписей
- ✅ Управление сессиями с тайм-аутами

## ⚙️ Требования

- **Rust**: 1.70+
- **Cargo**: последняя версия
- **wasm-pack**: для WASM сборки
- **WalletConnect Project ID**: получите на [cloud.walletconnect.com](https://cloud.walletconnect.com/)

## 📖 Дополнительные ресурсы

- [WalletConnect Documentation](https://docs.walletconnect.com/)
- [Reown Documentation](https://docs.reown.com/overview)
- [Qubic Wallet Spec](https://github.com/qubic/wallet-app/blob/main/walletconnect.md)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

## 🤝 Сравнение с TypeScript версией

| Функция | TypeScript | Rust |
|---------|-----------|------|
| Типобезопасность | ⚠️ Runtime | ✅ Compile-time |
| Производительность | ⚠️ V8 | ✅ Native |
| Размер бандла | ⚠️ ~200KB | ✅ ~100KB (WASM) |
| WASM поддержка | ❌ Нет | ✅ Да |
| Async/Await | ✅ Да | ✅ Да |
| Error handling | ⚠️ try/catch | ✅ Result<T, E> |

## 🐛 Отладка

### Включение логов

```rust
// В начале main()
tracing_subscriber::fmt::init();
```

### Проверка статуса

```rust
if client.is_session_active() {
    println!("✅ Connected");
} else {
    println!("❌ Not connected");
}
```

## 📝 Лицензия

Применяется та же лицензия, что и для основного проекта SCAPI.

## 🎉 Готово к использованию!

Библиотека полностью готова для интеграции в ваши Qubic dApp проекты. Начните с [Quick Start Guide](./WALLET_CONNECT_QUICKSTART.md) или изучите [примеры](./examples/).

---

**Создано с ❤️ для Qubic экосистемы**

