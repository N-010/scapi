# Устранение проблем WalletConnect

## Ошибка: "Соединение уже было установлено"

### Причины
1. **Повторное сканирование того же QR-кода** - каждый URI уникален и может быть использован только один раз
2. **Кэш в кошельке** - кошелек сохраняет историю подключений
3. **Не перезапущена программа** - старый URI остался в памяти

### Решение

#### 1. Перезапустите программу для нового URI
```bash
cargo run --bin scapi-cli
```

Каждый запуск создает **новый уникальный** URI с уникальным:
- `symKey` (случайные 32 байта)
- `topic` (SHA256 hash от symKey)
- `expiryTimestamp` (текущее время + 5 минут)

#### 2. Очистите кэш в кошельке

**В Qubic Wallet:**
1. Откройте настройки
2. Найдите раздел "WalletConnect" или "Подключения"
3. Удалите все старые сессии
4. Или перезапустите кошелек

#### 3. Проверьте уникальность подключения

Программа показывает уникальный ID:
```
🆔 Уникальный ID подключения: e10bf2f3a1c3b1bd656d3d0100b642ec...
```

Убедитесь, что каждый раз ID **разный**.

---

## Ошибка: "Неверный URL"

### Причины
1. **Неправильный формат URI** - должен соответствовать WalletConnect v2
2. **Отсутствуют обязательные параметры**
3. **Неправильный порядок параметров**

### Решение

URI должен иметь формат:
```
wc:<topic>@2?expiryTimestamp=<timestamp>&relay-protocol=irn&symKey=<key>
```

**Порядок параметров важен:**
1. `expiryTimestamp` - первый
2. `relay-protocol` - второй
3. `symKey` - третий

**Deep link для кошелька:**
```
qubic-wallet://pairwc/wc:<topic>@2?...
```

---

## Таймаут подключения

### Причины
1. **QR-код не был отсканирован** в течение 120 секунд
2. **Проблемы с сетью** - нет интернета
3. **Кошелек не поддерживает WalletConnect v2**

### Решение

1. **Сканируйте QR быстрее** - у вас есть 120 секунд
2. **Проверьте интернет** на телефоне и компьютере
3. **Обновите кошелек** до последней версии
4. **Используйте deep link** вместо QR-кода:
   ```
   qubic-wallet://pairwc/wc:...
   ```

---

## Проблемы с Project ID

### Симптомы
- Подключение не устанавливается
- Ошибки инициализации

### Решение

1. **Получите свой Project ID** на https://cloud.walletconnect.com/
2. **Установите переменную окружения:**
   ```bash
   # Windows PowerShell
   $env:WALLET_CONNECT_PROJECT_ID="your_project_id"
   
   # Linux/Mac
   export WALLET_CONNECT_PROJECT_ID="your_project_id"
   ```
3. **Проверьте, что Project ID активен** в панели WalletConnect

---

## Отладка

### Включите debug логирование
```bash
# Windows PowerShell
$env:RUST_LOG="debug"
cargo run --bin scapi-cli

# Linux/Mac
RUST_LOG=debug cargo run --bin scapi-cli
```

### Проверьте генерируемые значения
В debug режиме вы увидите:
```
Generated symKey: 1c66ce5bbdda58c1e56a18c093b2d416...
Calculated topic: e10bf2f3a1c3b1bd656d3d0100b642ec...
Expiry timestamp: 1761983605
```

Убедитесь, что:
- ✅ `symKey` - 64 символа (32 байта в hex)
- ✅ `topic` - 64 символа (SHA256 hash)
- ✅ `expiryTimestamp` - unix timestamp

---

## Контакты и поддержка

- **Документация WalletConnect:** https://docs.walletconnect.com/
- **Документация Reown:** https://docs.reown.com/
- **Qubic Wallet API:** https://github.com/qubic/wallet-app/blob/main/walletconnect.md

---

## Полезные команды

```bash
# Запуск с debug логами
RUST_LOG=debug cargo run --bin scapi-cli

# Сборка release версии (быстрее)
cargo build --release --bin scapi-cli
./target/release/scapi-cli

# Очистка и пересборка
cargo clean
cargo build --bin scapi-cli

# Запуск примеров
cargo run --example wallet_connect_basic
cargo run --example wallet_connect_transaction
```

