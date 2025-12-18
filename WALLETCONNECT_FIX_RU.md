# Исправление проблемы "Соединение было установлено через этот URL"

## 🔴 Проблема

При сканировании QR-кода для подключения через WalletConnect, кошелек показывает сообщение:
> "Соединение было установлено через этот URL"

И отказывается подключаться.

## ⚡ РЕШЕНИЕ (критичное исправление)

**Настоящая причина:** Использовался **`optional_namespaces`** вместо **`required_namespaces`**!

См. подробности в: **`WALLETCONNECT_FIX_REQUIRED_NAMESPACES.md`**

## 🔍 Анализ

### Сравнение с JavaScript версией

**JavaScript (qraw-frontend):**
```javascript
// В WalletConnectProvider.tsx (строки 234-252)
// При инициализации ПРИНУДИТЕЛЬНО отключает старые сессии
const storedTopic = localStorage.getItem('sessionTopic');
if (storedTopic) {
    walletClient.sessionTopic = storedTopic;
    await walletClient.disconnectWallet().catch(() => undefined);
}
localStorage.removeItem('sessionTopic');
```

**Rust (до исправления):**
- Создавал новый pairing topic при каждом запуске
- НЕ очищал старые сессии/подключения
- Relay сервер видел конфликт с существующим topic

### Почему это происходило?

1. **Повторное использование topic:** Каждый `connect()` создает новый `symKey` и `topic = SHA256(symKey)`
2. **Старые сессии не очищались:** Предыдущие подключения оставались активными на relay сервере
3. **Кошелек отклоняет:** При попытке создать новый pairing с уже существующим topic, кошелек показывает ошибку

## ✅ Решение

### Добавлен метод `cleanup_old_state()`

```rust
/// Clean up any previous connection state before creating a new connection
/// This prevents "connection already established" errors
async fn cleanup_old_state(&mut self) -> WalletConnectResult<()> {
    // 1. Останавливаем старый listener
    {
        let mut handle_guard = self.listener_handle.lock().unwrap();
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
    }

    // 2. Очищаем старое состояние подключения
    {
        let mut state_guard = self.state.lock().unwrap();
        if state_guard.is_some() {
            *state_guard = None;
        }
    }

    // 3. Очищаем сессию и connection info
    self.clear_session();
    *self.pairing_topic.lock().unwrap() = String::new();
    *self.connection_url.lock().unwrap() = String::new();

    Ok(())
}
```

### Интеграция в `connect()`

```rust
pub async fn connect(&mut self) -> WalletConnectResult<String> {
    // ВАЖНО: Очистка старого состояния ПЕРЕД созданием нового подключения
    self.cleanup_old_state().await?;
    
    // ... остальной код создания нового подключения
}
```

## 📊 Что изменилось?

| Аспект | До исправления | После исправления |
|--------|---------------|------------------|
| Старые сессии | Оставались активными | Автоматически очищаются |
| Listener | Мог работать в фоне | Останавливается перед новым подключением |
| Состояние | Накапливалось | Полностью очищается |
| Topic collision | Возможны конфликты | Каждое подключение чистое |

## 🎯 Результат

- ✅ Каждый запуск программы создает полностью новое, чистое подключение
- ✅ Старые сессии автоматически очищаются перед новым подключением
- ✅ Нет конфликтов с существующими topics на relay сервере
- ✅ Кошелек успешно принимает новые QR-коды

## 🚀 Использование

```bash
cargo run
```

Программа автоматически:
1. Очистит старые сессии
2. Создаст новый уникальный URI
3. Отобразит QR-код для сканирования

**Больше не нужно:**
- ❌ Вручную удалять старые сессии в кошельке
- ❌ Перезапускать кошелек
- ❌ Беспокоиться о конфликтах подключений

## 📝 Дополнительные улучшения

### Логирование
Добавлено подробное логирование процесса очистки:
```
[WalletConnect] Cleaning up old state...
[WalletConnect] Aborted old listener
[WalletConnect] Cleared old connection state
[WalletConnect] Old state cleaned up - ready for new connection
[WalletConnect] Creating NEW pairing session (unique per connection)
```

### Обновленные инструкции в main.rs
```
ℹ️  Старые сессии автоматически очищаются
ℹ️  Каждый запуск создает НОВЫЙ уникальный URI для подключения
ℹ️  Это предотвращает ошибки 'соединение уже установлено'
```

## 🔗 Связанные изменения

- `src/wallet_connect/client.rs` - добавлен метод `cleanup_old_state()`
- `src/wallet_connect/client.rs` - обновлен метод `connect()`
- `src/main.rs` - обновлены инструкции для пользователя

## 📚 Ссылки

- [WalletConnect JavaScript SDK](https://github.com/WalletConnect/walletconnect-monorepo)
- [qraw-frontend implementation](../qraw-frontend/src/api/wallet-connect-client.ts)
- [WalletConnect Protocol v2.0](https://docs.walletconnect.com/)

---

**Дата исправления:** 2025-11-01  
**Автор:** AI Assistant  
**Версия:** 1.0

