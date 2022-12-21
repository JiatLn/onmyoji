use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

use super::load_data;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub currency: Currency,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Currency {
    pub gold: u32,
    pub water: u32,
    pub purple: u32,
}

static JSON_PATH: &str = "../data/account.json";

impl Account {
    pub fn load() -> Result<Self> {
        let json_str = load_data::load_data(JSON_PATH)?;
        let account = serde_json::from_str(&json_str)?;
        Ok(account)
    }
    pub fn save(&self) -> Result<()> {
        let json_str = serde_json::to_string_pretty(&self)?;
        load_data::save_data(JSON_PATH, &json_str)?;
        Ok(())
    }
}

impl Currency {
    pub fn consum(&mut self, ty: &str, amount: u32) -> Result<()> {
        match ty {
            "gold" => {
                if self.gold < amount {
                    return Err(anyhow::anyhow!("not enough gold"));
                }
                self.gold -= amount;
            }
            "water" => {
                if self.water < amount {
                    return Err(anyhow::anyhow!("not enough water"));
                }
                self.water -= amount;
            }
            "purple" => {
                if self.purple < amount {
                    return Err(anyhow::anyhow!("not enough purple"));
                }
                self.purple -= amount;
            }
            _ => return Err(anyhow::anyhow!("unknown currency type")),
        }
        Ok(())
    }
}

impl Default for Account {
    fn default() -> Self {
        Self {
            currency: Currency::default(),
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            gold: 0,
            water: 0,
            purple: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_account() {
        let account = Account::load().unwrap();
        assert_eq!(account.currency.gold, 500);
    }
}
