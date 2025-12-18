#![cfg(not(target_arch = "wasm32"))]

mod sc_api;
mod wallet_connect;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("scapi-cli binary is not supported on wasm32 targets");
}

#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;
use wallet_connect::*;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<()> {
        // Инициализация логирования с DEBUG уровнем для детальной диагностики
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  🔗 WalletConnect для Qubic - Подключение по QR-коду      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Шаг 1: Конфигурация
    println!("📝 Шаг 1: Создание конфигурации WalletConnect...");

    let project_id = std::env::var("WALLET_CONNECT_PROJECT_ID")
        .unwrap_or_else(|_| {
            println!("⚠️  Переменная WALLET_CONNECT_PROJECT_ID не установлена");
            println!("   Используйте: export WALLET_CONNECT_PROJECT_ID=your_project_id");
            println!("   Или получите ID на: https://cloud.walletconnect.com/\n");
            "demo_project_id".to_string()
        });

    let config = WalletConnectConfig::new(project_id.clone(), "qubic:mainnet".to_string());

    println!("✅ Конфигурация создана");
    println!(
        "   Project ID: {}",
        if project_id == "demo_project_id" {
            "demo_project_id (замените на реальный!)"
        } else {
            &project_id
        }
    );
    println!("   Chain ID: qubic:mainnet\n");

    // Шаг 2: Создание клиента
    println!("🔧 Шаг 2: Создание WalletConnect клиента...");
    let mut client = WalletConnectClient::new(config);
    println!("✅ Клиент создан\n");

    // Шаг 3: Инициализация
    println!("🔌 Шаг 3: Инициализация клиента...");
    match client.init().await {
        Ok(_) => println!("✅ Клиент инициализирован успешно\n"),
        Err(e) => {
            println!("❌ Ошибка инициализации: {}\n", e);
            return Ok(());
        }
    }

    // Шаг 4: Генерация URI для QR-кода
    println!("📱 Шаг 4: Генерация URI для QR-кода...");
    println!("   ℹ️  Старые сессии автоматически очищаются");
    println!("   ℹ️  Каждый запуск создает НОВЫЙ уникальный URI для подключения");
    println!("   ℹ️  Это предотвращает ошибки 'соединение уже установлено'\n");

    let uri = match client.connect().await {
        Ok(uri) => {
            println!("✅ URI сгенерирован успешно\n");
            println!(
                "   🆔 Уникальный ID подключения: {}\n",
                uri.split('@').next().unwrap_or("").replace("wc:", "")
            );
            uri
        }
        Err(e) => {
            println!("❌ Ошибка генерации URI: {}\n", e);
            return Ok(());
        }
    };

    // Отображение QR-кода в консоли
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    QR КОД ДЛЯ СКАНИРОВАНИЯ                 ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Попытка отобразить QR-код в консоли
    #[cfg(debug_assertions)]
    {
        match qr2term::print_qr(&uri) {
            Ok(_) => println!("\n✅ QR-код отображен выше ⬆️\n"),
            Err(_) => {
                println!("⚠️  Не удалось отобразить QR-код в консоли\n");
                println!("📋 URI для ручного ввода или генерации QR:");
                println!("   {}\n", uri);
            }
        }
    }

    #[cfg(not(debug_assertions))]
    {
        println!("📋 WalletConnect URI:");
        println!("   {}\n", uri);
    }

    // Инструкции для пользователя
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                   КАК ПОДКЛЮЧИТЬСЯ                         ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!("\n1️⃣  Откройте ваш Qubic кошелек на телефоне");
    println!("2️⃣  Найдите функцию WalletConnect");
    println!("3️⃣  Отсканируйте QR-код выше");
    println!("4️⃣  Подтвердите подключение в кошельке\n");

    // Дополнительная информация
    println!("💡 Для deep link используйте:");
    println!("   qubic-wallet://pairwc/{}\n", uri);
    println!("💡 Для QR-кода онлайн:");
    println!("   https://www.qr-code-generator.com/\n");

    // Ожидание подключения кошелька с анимацией
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║              ⏳ ОЖИДАНИЕ ПОДКЛЮЧЕНИЯ КОШЕЛЬКА              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("   Отсканируйте QR-код в вашем Qubic кошельке");
    println!("   Таймаут: 120 секунд (2 минуты)\n");

    // Анимация ожидания в отдельной задаче
    let animation_handle = tokio::spawn(async {
        let mut dots = 0;
        let start = tokio::time::Instant::now();
        loop {
            let elapsed = start.elapsed().as_secs();
            let remaining = 120u64.saturating_sub(elapsed);

            if remaining == 0 {
                break;
            }

            dots = (dots + 1) % 4;
            let dots_str = ".".repeat(dots);
            let spaces_str = " ".repeat(3 - dots);

            print!(
                "\r   ⏳ Ожидание{}{} (осталось: {}с)    ",
                dots_str, spaces_str, remaining
            );
            std::io::Write::flush(&mut std::io::stdout()).ok();

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    // Ожидание подключения с timeout 120 секунд
    let connection_result = client.wait_for_connection(120).await;

    // Останавливаем анимацию
    animation_handle.abort();
    println!("\r                                                              \r");

    // Обработка результата подключения
    match connection_result {
        Ok(true) => {
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║              ✅ КОШЕЛЕК УСПЕШНО ПОДКЛЮЧЕН!                 ║");
            println!("╚════════════════════════════════════════════════════════════╝\n");

            if let Some(session) = client.get_session() {
                println!("📊 Информация о сессии:");
                println!("   Topic: {}", session.topic);
                if let Some(ttl) = session.time_until_expiry() {
                    println!("   Истекает через: {} часов", ttl / 3600);
                }
                println!("   Relay protocol: {}", session.relay_protocol);
                println!("   Статус: Активна ✅\n");
            }

            println!("💡 Дальше вы можете использовать методы клиента вручную:");
            println!("   • client.request_accounts()");
            println!("   • client.sign_message(...)");
            println!("   • client.send_transaction(...)");
            println!("   • client.disconnect()\n");
        }
        Ok(false) => {
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║              ❌ ПОДКЛЮЧЕНИЕ ОТКЛОНЕНО                      ║");
            println!("╚════════════════════════════════════════════════════════════╝\n");
            println!("   Подключение было отклонено в кошельке\n");
        }
        Err(e) => {
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║              ⏰ ТАЙМАУТ ОЖИДАНИЯ                           ║");
            println!("╚════════════════════════════════════════════════════════════╝\n");
            println!("   Ошибка: {}\n", e);
            println!("   Возможные причины:");
            println!("   • QR-код не был отсканирован");
            println!("   • Подключение было отклонено в кошельке");
            println!("   • Истек таймаут ожидания (120 сек)\n");

            println!("💡 Попробуйте:");
            println!("   • Запустите программу снова: cargo run");
            println!("   • Проверьте Project ID в переменной окружения");
            println!("   • Убедитесь, что кошелек поддерживает WalletConnect\n");
        }
    }

    // Информация о дальнейших действиях
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                  ДАЛЬНЕЙШИЕ ДЕЙСТВИЯ                       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("📚 Изучите документацию:");
    println!("   - WALLET_CONNECT_QUICKSTART.md - быстрый старт");
    println!("   - WALLET_CONNECT_API.md - полная документация");
    println!("   - examples/ - готовые примеры\n");

    println!("🧪 Запустите примеры:");
    println!("   cargo run --example wallet_connect_basic");
    println!("   cargo run --example wallet_connect_transaction");
    println!("   cargo run --example wallet_connect_events\n");

    println!("🌐 Для использования в браузере:");
    println!("   wasm-pack build --target web\n");

    println!("✨ Готово! Проект SCAPI с WalletConnect API готов к использованию!");

    Ok(())
}