# 🔍 Инструкции по отладке WalletConnect

## ✅ Что было добавлено

Добавлено **детальное логирование** для диагностики проблемы "Соединение было установлено через этот URL".

### 📊 Новые логи

Теперь вы увидите:

1. **При создании подключения:**
   ```
   [WalletConnect] Using REQUIRED namespaces (matching JS SDK)
   [WalletConnect] Chains: ["qubic:mainnet"]
   [WalletConnect] Methods: [...]
   [WalletConnect] 📤 Subscribing to pairing topic: <topic>
   [WalletConnect] 📤 Publishing SessionPropose to topic: <topic>
   [WalletConnect] ✅ SessionPropose sent successfully!
   [WalletConnect] 🔑 Pairing topic: <topic>
   [WalletConnect] ⏰ Expiry: <timestamp>
   ```

2. **При получении ответа от кошелька:**
   ```
   [WalletConnect] 📨 Received message: SessionProposeResponse on topic <topic>
   [WalletConnect] ✅ Received SessionProposeResponse from wallet!
   [WalletConnect] ✅ Wallet approved! Creating derived topic: <topic>
   ```

3. **При установке сессии:**
   ```
   [WalletConnect] 🎉 SessionSettle received! Session is being established...
   [WalletConnect] ✅ Session established successfully!
   ```

4. **При ошибках:**
   ```
   [WalletConnect] ❌ Received ERROR from wallet - code: <code>, message: '<message>'
   [WalletConnect] This error message suggests: <diagnosis>
   ```

## 🚀 Как тестировать

### 1. Запустите программу с логами

```bash
cd "D:\Work\MySelf\Qubic\SCAPI"
cargo run 2>&1 | tee walletconnect_debug.log
```

Или просто:
```bash
cargo run
```

Логи будут выводиться в консоль (DEBUG уровень включен).

### 2. Отсканируйте QR-код

- Откройте Qubic кошелек
- Найдите WalletConnect
- Отсканируйте QR-код

### 3. Наблюдайте за логами

Вы должны увидеть последовательность:

```
[WalletConnect] ✅ SessionPropose sent successfully!
[WalletConnect] 📨 Received message: ... on topic ...
```

**ВАЖНО:** Если вы видите ошибку от кошелька, она будет отображаться как:

```
[WalletConnect] ❌ Received ERROR from wallet - code: <code>, message: '<message>'
[WalletConnect] This error message suggests: Wallet sees an existing connection...
```

### 4. Что искать в логах

#### ✅ Успешное подключение:
```
[WalletConnect] ✅ Received SessionProposeResponse from wallet!
[WalletConnect] ✅ Wallet approved! Creating derived topic: ...
[WalletConnect] 🎉 SessionSettle received!
[WalletConnect] ✅ Session established successfully!
```

#### ❌ Ошибка "соединение уже установлено":
```
[WalletConnect] ❌ Received ERROR from wallet - code: <code>, message: 'Соединение было установлено через этот URL'
[WalletConnect] This error message suggests: Wallet sees an existing connection...
```

#### ⏰ Timeout (QR не отсканирован):
```
[WalletConnect] ⏰ Connection timeout - QR code not scanned
```

## 📋 Чек-лист диагностики

Проверьте следующие моменты в логах:

- [ ] **SessionPropose отправлен?**
  - Должна быть строка: `✅ SessionPropose sent successfully!`

- [ ] **Кошелек ответил?**
  - Должна быть строка: `📨 Received message: ...`
  - Если нет - проблема в доставке сообщения через relay

- [ ] **Какой тип ответа?**
  - `SessionProposeResponse` = ✅ кошелек одобрил
  - `Error` = ❌ кошелек отклонил
  - Нет ответа = ⏰ timeout или проблема с relay

- [ ] **Какая ошибка?**
  - Если есть `Error` с сообщением об "установленном соединении" - кошелек видит старый pairing
  - Проверьте код ошибки в логах

- [ ] **Используются правильные namespaces?**
  - Должна быть строка: `Using REQUIRED namespaces (matching JS SDK)`
  - Проверьте что `required_namespaces` содержит Qubic

## 🔧 Что делать если ошибка повторяется

### Если видите Error от кошелька:

1. **Скопируйте полный текст ошибки из логов**
   - Код ошибки
   - Сообщение ошибки

2. **Проверьте код ошибки:**
   - `5000` = User rejected (пользователь отклонил)
   - `5001` = Unauthorized (неавторизован)
   - Другие коды = специфичные ошибки WalletConnect

3. **Проверьте topic в URI:**
   - Убедитесь что topic меняется при каждом запуске
   - Если topic одинаковый - проблема в генерации

4. **Сравните с JavaScript версией:**
   - Запустите JavaScript версию
   - Сравните URI которые генерируются
   - Сравните SessionPropose сообщения

### Если не видите никаких ответов:

1. **Проверьте интернет соединение**
2. **Проверьте Project ID** - правильный ли он
3. **Проверьте relay сервер** - доступен ли он
4. **Проверьте логику listener_loop** - работает ли она

## 📝 Отправка логов для анализа

Если ошибка повторяется, соберите следующую информацию:

1. **Полный лог файл:**
   ```bash
   cargo run 2>&1 | tee walletconnect_debug.log
   ```

2. **URI который был сгенерирован** (первые 100 символов)

3. **Сообщение об ошибке от кошелька** (если есть)

4. **Время от создания URI до ошибки**

5. **Версия кошелька** (если известна)

## 🎯 Ожидаемый результат

После запуска с новым логированием вы должны четко видеть:

- ✅ Что именно отправляется кошельку
- ✅ Какой ответ получаем от кошелька  
- ✅ На каком этапе происходит ошибка
- ✅ Точное сообщение об ошибке с кодом

Это поможет понять **почему** кошелек отклоняет подключение.

---

**Следующий шаг:** Запустите `cargo run` и отсканируйте QR-код, затем проверьте логи!

