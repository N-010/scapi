use scapi::wallet_connect::events::*;
/// Пример обработки событий WalletConnect
///
/// Этот пример показывает:
/// - Регистрацию обработчиков событий
/// - Реагирование на различные события WalletConnect
/// - Управление жизненным циклом сессии
use scapi::wallet_connect::*;

// Кастомный обработчик событий
struct MyEventHandler {
    name: String,
}

impl EventCallback for MyEventHandler {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        println!("\n🔔 [{}] Событие получено:", self.name);
        println!("   Тип: {:?}", event);

        match event {
            WalletConnectEvent::SessionProposal => {
                println!("   📋 Предложение сессии");
                println!("   Данные: {}", payload);
            }
            WalletConnectEvent::SessionRequest => {
                println!("   📨 Запрос сессии");
                println!("   Данные: {}", payload);
            }
            WalletConnectEvent::SessionDelete => {
                println!("   🗑️  Сессия удалена");
                println!("   Причина: {}", payload);
            }
            WalletConnectEvent::SessionExpire => {
                println!("   ⏰ Сессия истекла");
                println!("   Детали: {}", payload);
            }
            WalletConnectEvent::ProposalExpire => {
                println!("   ⌛ Предложение истекло");
                println!("   Детали: {}", payload);
            }
            WalletConnectEvent::SessionEvent => {
                println!("   🎉 Событие сессии");
                println!("   Данные: {}", payload);
            }
            WalletConnectEvent::SessionUpdate => {
                println!("   🔄 Обновление сессии");
                println!("   Новые данные: {}", payload);
            }
            WalletConnectEvent::SessionExtend => {
                println!("   ⏭️  Продление сессии");
                println!("   Детали: {}", payload);
            }
            WalletConnectEvent::SessionPing => {
                println!("   🏓 Пинг сессии");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🎯 WalletConnect для Qubic - Обработка событий");
    println!("==============================================\n");

    // Создание конфигурации
    let config =
        WalletConnectConfig::new("your_project_id".to_string(), "qubic:mainnet".to_string());

    let mut client = WalletConnectClient::new(config);

    // Регистрация обработчиков событий
    println!("📝 Регистрация обработчиков событий...");

    // Логирующий обработчик (встроенный)
    client.event_handler().register(Box::new(LoggingCallback));

    // Кастомный обработчик
    client.event_handler().register(Box::new(MyEventHandler {
        name: "CustomHandler".to_string(),
    }));

    println!("✅ Обработчики зарегистрированы\n");

    // Инициализация
    println!("🔧 Инициализация клиента...");
    client.init().await?;
    println!("✅ Клиент инициализирован\n");

    // Генерация URI
    println!("📱 Генерация URI для подключения...");
    let uri = client.connect().await?;
    println!("✅ URI: {}\n", uri);

    println!("📲 Отсканируйте QR-код в кошельке для подключения");
    println!("   События будут отображаться здесь...\n");

    // Имитация ожидания событий
    println!("⏳ Ожидание событий (нажмите Ctrl+C для выхода)...\n");

    // В реальном приложении здесь был бы event loop
    // Для демонстрации просто ждем немного
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Симуляция некоторых событий для демонстрации
    println!("\n🧪 Симуляция событий для демонстрации:");

    // Событие подключения
    client.event_handler().emit(
        WalletConnectEvent::SessionProposal,
        serde_json::json!({
            "id": "test-proposal-123",
            "params": {}
        }),
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Событие обновления сессии
    client.event_handler().emit(
        WalletConnectEvent::SessionUpdate,
        serde_json::json!({
            "topic": "test-topic",
            "params": { "accounts": [] }
        }),
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Отключение
    println!("\n🔌 Отключение...");
    client.disconnect().await?;
    println!("✅ Отключено\n");

    println!("ℹ️  Пример завершен. В реальном приложении события будут");
    println!("   приходить от подключенного кошелька автоматически.");

    Ok(())
}
