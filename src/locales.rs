pub enum Language { En, Uk }
impl Language {
    pub fn from_code(code: Option<&str>) -> Self {
        match code { Some("uk") | Some("ua") => Language::Uk, _ => Language::En }
    }
    pub fn welcome_msg(&self) -> &'static str {
        match self {
            Language::En => "🛡️ <b>Welcome to SoulCode Core!</b>\n\nLow-level Web3 assistant for ultra-efficient token distribution.",
            Language::Uk => "🛡️ <b>Вітаємо в SoulCode Core!</b>\n\nТвій низькорівневий Web3-асистент для економного розсилання токенів.",
        }
    }
}