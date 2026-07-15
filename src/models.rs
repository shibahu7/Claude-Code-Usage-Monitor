use std::time::SystemTime;

#[derive(Clone, Debug, Default)]
pub struct UsageSection {
    pub percentage: f64,
    pub resets_at: Option<SystemTime>,
}

#[derive(Clone, Debug, Default)]
pub struct UsageData {
    pub session: UsageSection,
    pub weekly: UsageSection,
}

#[derive(Clone, Debug)]
pub struct CodexAccountUsage {
    pub account_id: String,
    pub label: String,
    pub usage: UsageData,
}

#[derive(Clone, Debug, Default)]
pub struct AppUsageData {
    pub claude_code: Option<UsageData>,
    pub codex_accounts: Vec<CodexAccountUsage>,
    pub antigravity: Option<UsageData>,
}
