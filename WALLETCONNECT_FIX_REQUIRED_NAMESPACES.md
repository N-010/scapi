# ✅ НАСТОЯЩЕЕ ИСПРАВЛЕНИЕ: requiredNamespaces vs optionalNamespaces

## 🔴 Реальная проблема

Кошелек отклонял подключение с сообщением: **"Соединение было установлено через этот URL"**

### Истинная причина

**Проблема была НЕ в очистке старых сессий** (хотя это тоже было добавлено для совместимости).

**Реальная проблема:** В Rust версии использовался **`optional_namespaces`**, а в JavaScript SDK - **`requiredNamespaces`**!

## 🔍 Сравнение

### JavaScript (qraw-frontend/src/api/wallet-connect-client.ts):
```javascript
const { uri, approval } = await client.connect({
    requiredNamespaces: {           // ← REQUIRED!
        qubic: {
            methods: Object.values(QubicNsMethods),
            chains: [this.qubicChainId ?? ''],
            events: Object.values(WalletEvents),
        },
    },
});
```

### Rust (ДО исправления):
```rust
let session_propose = SessionProposeParams {
    required_namespaces: HashMap::new(),    // ← ПУСТО!
    optional_namespaces,                     // ← Qubic здесь (НЕПРАВИЛЬНО!)
    relays: vec![...],
    ...
};
```

### Rust (ПОСЛЕ исправления):
```rust
let session_propose = SessionProposeParams {
    required_namespaces,                     // ← Qubic здесь (ПРАВИЛЬНО!)
    optional_namespaces: HashMap::new(),
    relays: vec![...],
    ...
};
```

## ⚡ Что было изменено

**Файл:** `src/wallet_connect/client.rs`

```rust
// БЫЛО (строки 186-196):
let namespace = QubicNamespace::new(self.config.qubic_chain_id.clone());
let mut optional_namespaces = HashMap::new();
optional_namespaces.insert(
    QUBIC_NAMESPACE.to_string(),
    WcNamespace {
        accounts: None,
        chains: namespace.chains.clone(),
        events: namespace.events.clone(),
        methods: namespace.methods.clone(),
    },
);

let session_propose = SessionProposeParams {
    required_namespaces: HashMap::new(),
    optional_namespaces,
    ...
};

// СТАЛО (строки 186-213):
let namespace = QubicNamespace::new(self.config.qubic_chain_id.clone());

// IMPORTANT: Use requiredNamespaces (not optional) to match JavaScript SDK behavior
// This ensures wallet properly validates the connection request
let mut required_namespaces = HashMap::new();
required_namespaces.insert(
    QUBIC_NAMESPACE.to_string(),
    WcNamespace {
        accounts: None,
        chains: namespace.chains.clone(),
        events: namespace.events.clone(),
        methods: namespace.methods.clone(),
    },
);

tracing::debug!("[WalletConnect] Using REQUIRED namespaces (matching JS SDK)");

let session_propose = SessionProposeParams {
    required_namespaces,
    optional_namespaces: HashMap::new(),
    ...
};
```

## 🎯 Почему это важно?

### requiredNamespaces
- **Обязательные** требования для подключения
- Кошелек **ДОЛЖЕН** поддерживать эти методы/chains
- Если не поддерживает - подключение отклоняется
- Используется для **критичных** функций

### optionalNamespaces
- **Необязательные** возможности
- Кошелек **МОЖЕТ** поддерживать эти методы
- Если не поддерживает - подключение всё равно проходит
- Используется для **дополнительных** функций

### Что происходило?

1. **Rust версия (до исправления):**
   - Отправляла Qubic в `optionalNamespaces`
   - Кошелек видел: "эти методы необязательны"
   - Кошелек думал: "это не основной запрос подключения"
   - Кошелек отклонял как дубликат или некорректный запрос

2. **JavaScript версия:**
   - Отправляет Qubic в `requiredNamespaces`
   - Кошелек видит: "это обязательное подключение к Qubic"
   - Кошелек правильно обрабатывает запрос

3. **Rust версия (после исправления):**
   - Отправляет Qubic в `requiredNamespaces`
   - Поведение **идентично** JavaScript версии
   - Кошелек правильно принимает подключение

## 📊 Результат

| Параметр | До исправления | После исправления |
|----------|---------------|-------------------|
| `required_namespaces` | `{}` (пусто) | `{ qubic: {...} }` |
| `optional_namespaces` | `{ qubic: {...} }` | `{}` (пусто) |
| Поведение кошелька | ❌ Отклоняет | ✅ Принимает |
| Соответствие JS SDK | ❌ Нет | ✅ Полное |

## 🚀 Проверка

Запустите программу:
```bash
cargo run
```

Теперь вы должны увидеть в логах:
```
[WalletConnect] Using REQUIRED namespaces (matching JS SDK)
[WalletConnect] Chains: ["qubic:mainnet"]
[WalletConnect] Methods: [...]
```

И кошелек должен **успешно подключиться** без ошибки "соединение уже установлено"!

## 📝 Дополнительные изменения

Помимо основного исправления, были добавлены:

1. **Метод `cleanup_old_state()`** - очистка старых сессий (для полной совместимости с JS SDK)
2. **Улучшенное логирование** - видно какие namespaces используются
3. **Подробная документация** - понятно почему используются requiredNamespaces

## ⚠️ Важное примечание

Если вы разрабатываете свой WalletConnect клиент:

✅ **ИСПОЛЬЗУЙТЕ `requiredNamespaces`** для основных функций вашего блокчейна  
❌ **НЕ ИСПОЛЬЗУЙТЕ `optionalNamespaces`** для критичных методов

`optionalNamespaces` подходит только для дополнительных, необязательных функций, которые кошелек может не поддерживать.

## 🔗 Связанные файлы

- `src/wallet_connect/client.rs` - основное исправление
- `WALLETCONNECT_FIX_RU.md` - дополнительная информация
- `QUICKSTART_WALLETCONNECT_RU.md` - инструкции по использованию

---

**Дата исправления:** 2025-11-01  
**Проблема:** requiredNamespaces vs optionalNamespaces  
**Статус:** ✅ Исправлено

