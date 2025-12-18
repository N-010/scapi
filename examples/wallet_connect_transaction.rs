/// Пример отправки транзакции через WalletConnect
///
/// Этот пример показывает:
/// - Подключение к кошельку
/// - Запрос аккаунтов
/// - Создание и отправку транзакции
use scapi::wallet_connect::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("💸 WalletConnect для Qubic - Отправка транзакции");
    println!("================================================\n");

    // Настройка клиента
    let metadata = ClientMetadata {
        name: "Qubic Transaction Example".to_string(),
        description: "Пример отправки транзакции через WalletConnect".to_string(),
        url: "https://qubic.org".to_string(),
        icons: vec!["https://qx.qubic.org/assets/icons/favicon.ico".to_string()],
    };

    let config =
        WalletConnectConfig::new("your_project_id".to_string(), "qubic:mainnet".to_string())
            .with_metadata(metadata);

    let mut client = WalletConnectClient::new(config);

    // Инициализация
    println!("🔧 Инициализация клиента...");
    client.init().await?;
    println!("✅ Клиент готов к работе\n");

    // Генерация QR
    println!("📱 Генерация QR-кода...");
    let uri = client.connect().await?;
    println!("✅ Отсканируйте QR-код:");
    println!("   {}\n", uri);

    println!("⏳ Ожидание подключения...");
    // В реальном приложении здесь будет ожидание события подключения

    // Симуляция успешного подключения
    if !client.is_session_active() {
        println!("⚠️  Для выполнения транзакции необходимо подключение к кошельку");
        return Ok(());
    }

    // Запрос аккаунтов
    println!("\n📋 Запрос списка аккаунтов...");
    match client.request_accounts().await {
        Ok(accounts) => {
            println!("✅ Найдено аккаунтов: {}", accounts.len());
            for (i, account) in accounts.iter().enumerate() {
                println!("   {}. {}", i + 1, account.address);
                if let Some(balance) = account.amount {
                    println!("      Баланс: {} Qubic", balance);
                }
            }

            if accounts.is_empty() {
                println!("⚠️  Нет доступных аккаунтов");
                return Ok(());
            }

            // Создание транзакции
            println!("\n💰 Создание транзакции...");

            let from = accounts[0].address.clone();
            let to = get_recipient_address()?;
            let amount = get_amount()?;

            let tx_params = QubicTransactionParams {
                from: from.clone(),
                to: to.clone(),
                amount,
                tick: None,
                input_type: Some(0),
                payload: None,
            };

            println!("\n📄 Параметры транзакции:");
            println!("   От:    {}", from);
            println!("   Кому:  {}", to);
            println!("   Сумма: {} Qubic", amount);

            // Подтверждение
            print!("\n❓ Отправить транзакцию? (y/n): ");
            io::stdout().flush()?;
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;

            if confirm.trim().to_lowercase() != "y" {
                println!("❌ Транзакция отменена");
                return Ok(());
            }

            // Отправка транзакции
            println!("\n🚀 Отправка транзакции...");
            println!("   (Подтвердите транзакцию в вашем кошельке)");

            match client.send_transaction(tx_params).await {
                Ok(result) => {
                    println!("✅ Транзакция успешно отправлена!");
                    println!("   Результат: {:?}", result);
                }
                Err(e) => {
                    println!("❌ Ошибка отправки транзакции: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Ошибка получения аккаунтов: {}", e);
        }
    }

    // Отключение
    println!("\n🔌 Отключение от кошелька...");
    client.disconnect().await?;
    println!("✅ Отключено");

    Ok(())
}

fn get_recipient_address() -> Result<String, Box<dyn std::error::Error>> {
    print!("\n📬 Введите адрес получателя: ");
    io::stdout().flush()?;
    let mut address = String::new();
    io::stdin().read_line(&mut address)?;
    Ok(address.trim().to_string())
}

fn get_amount() -> Result<u64, Box<dyn std::error::Error>> {
    print!("💵 Введите сумму (в минимальных единицах): ");
    io::stdout().flush()?;
    let mut amount = String::new();
    io::stdin().read_line(&mut amount)?;
    Ok(amount.trim().parse()?)
}
