# WalletConnect Module для Qubic

Этот модуль предоставляет полную реализацию WalletConnect v2 протокола для Qubic blockchain.

## 📁 Структура модуля

```
wallet_connect/
├── mod.rs              # Главный модуль, экспорты
├── types.rs            # Типы данных и структуры
├── client.rs           # Основной WalletConnect клиент
├── session.rs          # Управление сессиями
├── events.rs           # Обработка событий
├── qubic_namespace.rs  # Qubic-специфичные методы
└── wasm_bindings.rs    # WASM биндинги для браузера
```

## 🔑 Ключевые компоненты

### WalletConnectClient

Главный класс для взаимодействия с WalletConnect:

```rust
use scapi::wallet_connect::*;

let config = WalletConnectConfig::new(
    "project_id".to_string(),
    "qubic:mainnet".to_string()
);

let mut client = WalletConnectClient::new(config);
client.init().await?;
```

### Types

Основные типы данных:
- `WalletConnectConfig` - конфигурация клиента
- `WalletAccount` - информация об аккаунте
- `QubicTransactionParams` - параметры транзакции
- `SignatureResponse` - ответ с подписью
- `WalletConnectionStatus` - статус подключения

### Events

Система событий для реагирования на изменения:
- `WalletConnectEvent` - типы событий
- `EventCallback` - trait для обработчиков
- `EventHandler` - менеджер событий

### Qubic Namespace

Qubic-специфичные методы WalletConnect:
- `qubic_requestAccounts` - запрос аккаунтов
- `qubic_sendQubic` - отправка Qubic
- `qubic_signTransaction` - подпись транзакции
- `qubic_sendTransaction` - отправка транзакции
- `qubic_sign` - подпись сообщения

## 🎯 Основные возможности

### 1. Подключение к кошельку

```rust
let uri = client.connect().await?;
// Отобразите uri как QR-код
```

### 2. Запрос аккаунтов

```rust
let accounts = client.request_accounts().await?;
for account in accounts {
    println!("Address: {}", account.address);
}
```

### 3. Отправка транзакции

```rust
let params = QubicTransactionParams {
    from: "SENDER_ADDRESS".to_string(),
    to: "RECIPIENT_ADDRESS".to_string(),
    amount: 1_000_000,
    tick: None,
    input_type: Some(0),
    payload: None,
};

let result = client.send_transaction(params).await?;
```

### 4. Обработка событий

```rust
use scapi::wallet_connect::events::*;

struct MyHandler;

impl EventCallback for MyHandler {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        println!("Event: {:?}", event);
    }
}

client.event_handler().register(Box::new(MyHandler));
```

## 🌐 WASM Support

Модуль полностью поддерживает компиляцию в WebAssembly для использования в браузере:

```bash
wasm-pack build --target web
```

JavaScript/TypeScript использование:

```javascript
import { WalletConnectClient } from './pkg/scapi.js';

const client = new WalletConnectClient('project_id', 'qubic:mainnet');
await client.initClient();
const uri = await client.genConnectUrl();
```

## 🔒 Безопасность

Модуль следует лучшим практикам безопасности:
- Все криптографические операции выполняются в кошельке
- Приватные ключи никогда не передаются через сеть
- Поддержка проверки подписей
- Безопасное управление сессиями

## 📊 Архитектура

```
┌─────────────────┐
│   Your dApp     │
└────────┬────────┘
         │
    ┌────▼────────────────────┐
    │ WalletConnectClient     │
    ├─────────────────────────┤
    │ - connect()             │
    │ - request_accounts()    │
    │ - send_transaction()    │
    └────────┬────────────────┘
             │
    ┌────────▼────────────────┐
    │   Event System          │
    │   (EventHandler)        │
    └────────┬────────────────┘
             │
    ┌────────▼────────────────┐
    │   Session Manager       │
    └────────┬────────────────┘
             │
    ┌────────▼────────────────┐
    │  WalletConnect Relay    │
    │  (relay.walletconnect)  │
    └────────┬────────────────┘
             │
    ┌────────▼────────────────┐
    │   Qubic Wallet App      │
    └─────────────────────────┘
```

## 🧪 Тестирование

Запустите примеры для тестирования:

```bash
# Базовое подключение
cargo run --example wallet_connect_basic

# Отправка транзакции
cargo run --example wallet_connect_transaction

# Обработка событий
cargo run --example wallet_connect_events
```

## 📖 Дальнейшее чтение

- [API Documentation](../../WALLET_CONNECT_API.md)
- [Quick Start Guide](../../WALLET_CONNECT_QUICKSTART.md)
- [WalletConnect Docs](https://docs.walletconnect.com/)
- [Qubic Wallet Spec](https://github.com/qubic/wallet-app/blob/main/walletconnect.md)

## 🤝 Вклад в разработку

При добавлении новых функций:
1. Следуйте существующей структуре кода
2. Добавляйте документацию
3. Обновляйте примеры
4. Тестируйте на WASM и native

## 📝 История изменений

### v0.1.0 (текущая)
- ✅ Базовая реализация WalletConnect v2
- ✅ Поддержка Qubic namespace
- ✅ WASM биндинги
- ✅ Система событий
- ✅ Управление сессиями
- ✅ Примеры и документация

