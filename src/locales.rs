pub enum Language {
    En,
    Uk,
}

impl Language {
    pub fn from_code(code: Option<&str>) -> Self {
        match code {
            Some("uk") | Some("ua") => Language::Uk,
            _ => Language::En, 
        }
    }

    pub fn welcome_msg(&self) -> &'static str {
        match self {
            Language::En => " <b>Welcome to SoulCode Core!</b>\n\nYour low-level Web3 assistant for ultra-efficient token distribution.\n\n⚡️ Save up to 45% on Gas fees using our Yul/Assembly core.",
            Language::Uk => " <b>Вітаємо в SoulCode Core!</b>\n\nТвій низькорівневий Web3-асистент для ультра-економного масового розсилання токенів.\n\n⚡️ Економте до 45% газу завдяки нашому ядру на Yul/Assembly.",
        }
    }

    pub fn btn_create_airdrop(&self) -> &'static str {
        match self {
            Language::En => " Create Bulk Transfer",
            Language::Uk => " Створити розсилку",
        }
    }

    pub fn btn_my_subscription(&self) -> &'static str {
        match self {
            Language::En => " Subscription & Tariff",
            Language::Uk => " Підписка та Тариф",
        }
    }

    pub fn btn_get_contract(&self) -> &'static str {
        match self {
            Language::En => " Get Yul Contract Code",
            Language::Uk => " Отримати код Yul-контракту",
        }
    }

    pub fn err_mismatch(&self) -> &'static str {
        match self {
            Language::En => " <b>Error:</b> The number of addresses and amounts does not match!",
            Language::Uk => " <b>Помилка:</b> Кількість адрес та сум не збігається!",
        }
    }

    pub fn success_encoded(&self) -> &'static str {
        match self {
            Language::En => " <b>Success!</b> Your gas-optimized calldata is ready.\nCopy the code below to sign via Wallet:",
            Language::Uk => " <b>Успіх!</b> Твій економний calldata згенеровано.\nСкопіюй код нижче для підпису через гаманець:",
        }
    }

    pub fn subscription_info(&self) -> &'static str {
        match self {
            Language::En => "<b>💎 SoulCode Pro Plan</b>\n\n• Unlimited Calldata Generation\n• Access to Yul/Assembly Contracts\n• Priority gas-optimization updates\n\nPrice: <b>0.05 ETH / month</b>",
            Language::Uk => "<b>💎 Тариф SoulCode Pro</b>\n\n• Безлімітна генерація Calldata\n• Доступ до Yul/Assembly контрактів\n• Пріоритетні оновлення оптимізації газу\n\nЦіна: <b>0.05 ETH / місяць</b>",
        }
    }
}