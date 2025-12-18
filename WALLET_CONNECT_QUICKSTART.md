# WalletConnect для Qubic - Быстрый старт

## 🚀 5-минутная интеграция

### Шаг 1: Получение Project ID

1. Зайдите на [WalletConnect Cloud](https://cloud.walletconnect.com/)
2. Создайте новый проект
3. Скопируйте **Project ID**

### Шаг 2: Установка

Добавьте в ваш проект:

```rust
// В вашем main.rs или lib.rs
use scapi::wallet_connect::*;
```

### Шаг 3: Базовый код

```rust
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Конфигурация
    let config = WalletConnectConfig::new(
        "YOUR_PROJECT_ID".to_string(),
        "qubic:mainnet".to_string()
    );

    // 2. Создание клиента
    let mut client = WalletConnectClient::new(config);
    
    // 3. Инициализация
    client.init().await?;
    
    // 4. Получение QR-кода URI
    let uri = client.connect().await?;
    println!("QR Code URI: {}", uri);
    
    // 5. После сканирования - запрос аккаунтов
    let accounts = client.request_accounts().await?;
    println!("Connected: {:?}", accounts);
    
    Ok(())
}
```

### Шаг 4: Запуск

```bash
cargo run --example wallet_connect_basic
```

## 🌐 Использование в браузере

### Шаг 1: Сборка WASM

```bash
wasm-pack build --target web
```

### Шаг 2: HTML

```html
<!DOCTYPE html>
<html>
<head>
    <title>Qubic WalletConnect</title>
    <script src="https://cdn.jsdelivr.net/npm/qrcode/build/qrcode.min.js"></script>
</head>
<body>
    <div id="qrcode"></div>
    <button id="connect">Connect Wallet</button>
    <div id="status"></div>

    <script type="module">
        import init, { WalletConnectClient } from './pkg/scapi.js';

        async function main() {
            await init();

            const client = new WalletConnectClient(
                'YOUR_PROJECT_ID',
                'qubic:mainnet'
            );

            document.getElementById('connect').onclick = async () => {
                await client.initClient();
                const uri = await client.genConnectUrl();
                
                // Показать QR-код
                QRCode.toCanvas(
                    document.getElementById('qrcode'),
                    uri,
                    (error) => {
                        if (error) console.error(error);
                    }
                );

                document.getElementById('status').innerText = 
                    'Scan QR code in your Qubic wallet';
            };
        }

        main();
    </script>
</body>
</html>
```

## 📱 Поддерживаемые кошельки

- **Qubic Wallet App** (iOS, Android)
- Любой кошелек с поддержкой WalletConnect v2

## 🔐 Безопасность

✅ **Рекомендуется:**
- Используйте HTTPS для production
- Храните Project ID в переменных окружения
- Проверяйте подписи транзакций

❌ **Не делайте:**
- Не храните приватные ключи в коде
- Не отправляйте транзакции без подтверждения пользователя

## 🐛 Отладка

### Включение логов

```rust
// Добавьте в начало main()
tracing_subscriber::fmt::init();
```

### Проверка подключения

```rust
if client.is_session_active() {
    println!("✅ Connected");
} else {
    println!("❌ Not connected");
}
```

## 📚 Дополнительные ресурсы

- [Полная документация](./WALLET_CONNECT_API.md)
- [Примеры кода](./examples/)
- [Qubic Wallet Spec](https://github.com/qubic/wallet-app/blob/main/walletconnect.md)

## 💬 Поддержка

Если у вас возникли вопросы:
1. Проверьте [документацию](./WALLET_CONNECT_API.md)
2. Посмотрите [примеры](./examples/)
3. Создайте issue в репозитории

## 🎉 Готово!

Теперь ваше приложение может подключаться к Qubic кошелькам через WalletConnect!

