# WalletConnect API для Qubic

Rust библиотека для подключения dApp к Qubic кошельку через WalletConnect v2 с поддержкой QR-кодов.

## 📋 Содержание

- [Установка](#установка)
- [Быстрый старт](#быстрый-старт)
- [Использование в браузере (WASM)](#использование-в-браузере-wasm)
- [API Reference](#api-reference)
- [Примеры](#примеры)

## 🚀 Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
scapi = { path = "path/to/scapi" }
tokio = { version = "1", features = ["full"] }
```

## ⚡ Быстрый старт

### Rust (Native)

```rust
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), WalletConnectError> {
    // 1. Создание конфигурации
    let config = WalletConnectConfig::new(
        "your_project_id".to_string(),
        "qubic:mainnet".to_string()
    );

    // 2. Создание клиента
    let mut client = WalletConnectClient::new(config);

    // 3. Инициализация
    client.init().await?;

    // 4. Генерация URI для QR-кода
    let uri = client.connect().await?;
    println!("Scan this QR code: {}", uri);

    // 5. Ожидание подключения кошелька
    // (в реальном приложении это будет автоматически после сканирования QR)

    // 6. Запрос аккаунтов
    let accounts = client.request_accounts().await?;
    println!("Connected accounts: {:?}", accounts);

    // 7. Отправка транзакции
    let params = QubicTransactionParams {
        from: accounts[0].address.clone(),
        to: "BQYUPNXEQVBUQWILQGPUZTJEKXEZRAAQXSUIAQRPESWKQTZLYWGFEDMXKZLA".to_string(),
        amount: 1_000_000,
        tick: None,
        input_type: None,
        payload: None,
    };

    let result = client.send_transaction(params).await?;
    println!("Transaction sent: {:?}", result);

    Ok(())
}
```

## 🌐 Использование в браузере (WASM)

### Сборка WASM

```bash
# Установка wasm-pack
cargo install wasm-pack

# Сборка
wasm-pack build --target web --out-dir pkg
```

### JavaScript/TypeScript

```javascript
import init, { WalletConnectClient } from './pkg/scapi.js';

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

    // Генерация URI для QR-кода
    const uri = await client.genConnectUrl();
    
    // Отображение QR-кода (используйте любую JS библиотеку для QR)
    displayQRCode(uri);

    // После сканирования QR кошельком, запросить аккаунты
    const accounts = await client.requestAccounts();
    console.log('Connected accounts:', accounts);

    // Отправка транзакции
    const txParams = {
        from: accounts[0].address,
        to: 'BQYUPNXEQVBUQWILQGPUZTJEKXEZRAAQXSUIAQRPESWKQTZLYWGFEDMXKZLA',
        amount: 1000000,
        inputType: 0,
        payload: null
    };

    const result = await client.sendTransaction(txParams);
    console.log('Transaction result:', result);
}

// Вспомогательная функция для отображения QR
function displayQRCode(uri) {
    // Используйте библиотеку типа qrcode.react или qrcode
    // Пример с qrcode:
    import QRCode from 'qrcode';
    
    QRCode.toCanvas(document.getElementById('qrcode'), uri, (error) => {
        if (error) console.error(error);
        console.log('QR code generated!');
    });
}
```

## 📚 API Reference

### WalletConnectClient

#### Создание клиента

```rust
pub fn new(config: WalletConnectConfig) -> Self
```

Создает новый экземпляр WalletConnect клиента.

**Параметры:**
- `config: WalletConnectConfig` - конфигурация клиента

#### Инициализация

```rust
pub async fn init(&mut self) -> WalletConnectResult<()>
```

Инициализирует клиент и подключается к relay серверу.

#### Генерация URI для подключения

```rust
pub async fn connect(&mut self) -> WalletConnectResult<String>
```

Генерирует WalletConnect URI для отображения в QR-коде.

**Возвращает:** URI строку для QR-кода

#### Запрос аккаунтов

```rust
pub async fn request_accounts(&self) -> WalletConnectResult<Vec<WalletAccount>>
```

Запрашивает список аккаунтов из подключенного кошелька.

**Возвращает:** Вектор объектов `WalletAccount`

#### Отправка Qubic

```rust
pub async fn send_qubic(
    &self,
    from: &str,
    to: &str,
    amount: u64,
) -> WalletConnectResult<serde_json::Value>
```

Отправляет простой перевод Qubic.

**Параметры:**
- `from: &str` - адрес отправителя
- `to: &str` - адрес получателя
- `amount: u64` - сумма в минимальных единицах

#### Подписание транзакции

```rust
pub async fn sign_transaction(
    &self,
    params: QubicTransactionParams,
) -> WalletConnectResult<SignatureResponse>
```

Подписывает транзакцию без отправки в сеть.

**Параметры:**
- `params: QubicTransactionParams` - параметры транзакции

**Возвращает:** Объект `SignatureResponse` с подписью

#### Отправка транзакции

```rust
pub async fn send_transaction(
    &self,
    params: QubicTransactionParams,
) -> WalletConnectResult<serde_json::Value>
```

Подписывает и отправляет транзакцию в сеть.

**Параметры:**
- `params: QubicTransactionParams` - параметры транзакции

#### Подписание сообщения

```rust
pub async fn sign_message(
    &self,
    from: &str,
    message: &str,
) -> WalletConnectResult<SignatureResponse>
```

Подписывает произвольное сообщение.

**Параметры:**
- `from: &str` - адрес подписывающего
- `message: &str` - сообщение для подписи

#### Отключение

```rust
pub async fn disconnect(&mut self) -> WalletConnectResult<()>
```

Отключает кошелек и очищает сессию.

### Структуры данных

#### WalletConnectConfig

```rust
pub struct WalletConnectConfig {
    pub project_id: String,
    pub qubic_chain_id: String,
    pub metadata: ClientMetadata,
    pub relay_url: Option<String>,
}
```

**Методы:**
- `new(project_id, qubic_chain_id) -> Self`
- `with_metadata(metadata) -> Self`
- `with_relay_url(url) -> Self`

#### QubicTransactionParams

```rust
pub struct QubicTransactionParams {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub tick: Option<u64>,
    pub input_type: Option<u16>,
    pub payload: Option<String>,
}
```

#### WalletAccount

```rust
pub struct WalletAccount {
    pub address: String,
    pub amount: Option<u64>,
    pub additional_data: HashMap<String, serde_json::Value>,
}
```

## 🔧 Примеры

### Пример 1: Простое подключение

```rust
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WalletConnectConfig::new(
        "your_project_id".to_string(),
        "qubic:mainnet".to_string()
    );

    let mut client = WalletConnectClient::new(config);
    client.init().await?;

    let uri = client.connect().await?;
    println!("Connect with: {}", uri);

    Ok(())
}
```

### Пример 2: Отправка транзакции

```rust
use scapi::wallet_connect::*;

async fn send_qubic_transaction(
    client: &WalletConnectClient,
    from: &str,
    to: &str,
    amount: u64,
) -> Result<(), WalletConnectError> {
    let params = QubicTransactionParams {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        tick: None,
        input_type: Some(0),
        payload: None,
    };

    let result = client.send_transaction(params).await?;
    println!("Transaction result: {:?}", result);

    Ok(())
}
```

### Пример 3: Обработка событий

```rust
use scapi::wallet_connect::*;
use scapi::wallet_connect::events::*;

struct MyEventHandler;

impl EventCallback for MyEventHandler {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        match event {
            WalletConnectEvent::SessionDelete => {
                println!("Session deleted!");
            }
            WalletConnectEvent::SessionExpire => {
                println!("Session expired!");
            }
            _ => {
                println!("Event: {:?}, Payload: {:?}", event, payload);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WalletConnectConfig::new(
        "your_project_id".to_string(),
        "qubic:mainnet".to_string()
    );

    let client = WalletConnectClient::new(config);
    
    // Регистрация обработчика событий
    client.event_handler().register(Box::new(MyEventHandler));

    Ok(())
}
```

### Пример 4: Кастомная метадата

```rust
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = ClientMetadata {
        name: "My Qubic dApp".to_string(),
        description: "Awesome Qubic Application".to_string(),
        url: "https://my-dapp.com".to_string(),
        icons: vec!["https://my-dapp.com/icon.png".to_string()],
    };

    let config = WalletConnectConfig::new(
        "your_project_id".to_string(),
        "qubic:mainnet".to_string()
    )
    .with_metadata(metadata);

    let mut client = WalletConnectClient::new(config);
    client.init().await?;

    Ok(())
}
```

## 🔗 Интеграция с TypeScript приложением

Если вы переходите с TypeScript версии на Rust, вот сравнение API:

### TypeScript (старая версия)

```typescript
import WalletConnectClient from './api/wallet-connect-client';

const client = new WalletConnectClient();
await client.initClient();
const { uri } = await client.genConnectUrl();
const accounts = await client.requestAccounts();
await client.sendTransaction(params);
```

### Rust WASM (новая версия)

```javascript
import { WalletConnectClient } from './pkg/scapi.js';

const client = new WalletConnectClient('project_id', 'qubic:mainnet');
await client.initClient();
const uri = await client.genConnectUrl();
const accounts = await client.requestAccounts();
await client.sendTransaction(params);
```

## 📖 Дополнительная документация

- [WalletConnect Documentation](https://docs.walletconnect.com/)
- [Reown Documentation](https://docs.reown.com/overview)
- [Qubic Wallet Documentation](https://github.com/qubic/wallet-app/blob/main/walletconnect.md)

## 🛠️ Требования

- Rust 1.70+
- Cargo
- wasm-pack (для WASM сборки)

## ⚠️ Важные замечания

1. **Project ID**: Получите свой Project ID на [WalletConnect Cloud](https://cloud.walletconnect.com/)
2. **Chain ID**: Для Qubic используйте формат `qubic:mainnet` или `qubic:testnet`
3. **Безопасность**: Никогда не храните приватные ключи в клиентском коде
4. **CORS**: При использовании в браузере убедитесь, что CORS настроен правильно

## 📝 Лицензия

Как часть проекта SCAPI, применяется та же лицензия.

## 🤝 Вклад

Contributions are welcome! Пожалуйста, создайте issue или pull request.

