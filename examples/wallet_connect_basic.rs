/// Базовый пример использования WalletConnect API для Qubic
///
/// Этот пример показывает:
/// - Создание и инициализацию клиента
/// - Генерацию QR-кода для подключения
/// - Базовые операции с кошельком
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования (опционально)
    tracing_subscriber::fmt::init();

    println!("🚀 WalletConnect для Qubic - Базовый пример");
    println!("===========================================\n");

    // Шаг 1: Создание конфигурации
    println!("📝 Шаг 1: Создание конфигурации...");
    let config = WalletConnectConfig::new(
        "your_walletconnect_project_id".to_string(), // Замените на ваш Project ID
        "qubic:mainnet".to_string(),
    );
    println!("✅ Конфигурация создана\n");

    // Шаг 2: Создание клиента
    println!("🔧 Шаг 2: Создание клиента...");
    let mut client = WalletConnectClient::new(config);
    println!("✅ Клиент создан\n");

    // Шаг 3: Инициализация
    println!("🔌 Шаг 3: Инициализация клиента...");
    client.init().await?;
    println!("✅ Клиент инициализирован\n");

    // Шаг 4: Генерация URI для QR-кода
    println!("📱 Шаг 4: Генерация URI для подключения...");
    let uri = client.connect().await?;
    println!("✅ URI сгенерирован:");
    println!("   {}\n", uri);

    println!("📲 Отсканируйте этот QR-код в вашем Qubic кошельке:");
    println!("   (В реальном приложении здесь будет QR-код)\n");

    // В реальном приложении здесь бы ожидали подключения кошелька
    println!("⏳ Ожидание подключения кошелька...");
    println!("   (Это автоматически произойдет после сканирования QR)\n");

    // Примечание: Следующие шаги требуют реального подключения к кошельку
    println!("ℹ️  Дальнейшие операции (запрос аккаунтов, транзакции) требуют");
    println!("   подключения к реальному Qubic кошельку через WalletConnect");

    Ok(())
}
