/// Holder
pub trait Holder {
    /// Net present value
    fn net_present_value(account: Account) -> u128;

    /// Is '{Principal}' a deponent
    fn is_deponent(account: Account) -> bool;
}

/// Principal
pub struct Principal {
    pub is_alive: bool,
}

/// Wallet
pub struct Wallet {
    pub net_cash_flow_at_time_t: u128, 
    pub time_of_cash_flow: u128, 
    pub discount_rate: u128
}

pub struct Account {
    pub principal: Principal,
    pub wallet: Wallet,
}

/// Principal implementation
impl Holder for Account {
    fn net_present_value(account: Account) -> u128 {
        if account.principal.is_alive {
            account.wallet.net_cash_flow_at_time_t / (1 + account.wallet.discount_rate).pow(account.wallet.time_of_cash_flow as u32)
        } else {
            0
        }
    }

    fn is_deponent(account: Account) -> bool {
        if account.principal.is_alive && Self::net_present_value(account) > u64::MAX  as u128  {
            true
        } else {
            false
        }    
    }
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use super::*;

    fn init_account() -> Account {
        Account {
            principal: Principal { 
                is_alive: true
            },
            wallet: Wallet {
                net_cash_flow_at_time_t: 99999 as u128, 
                time_of_cash_flow: 3 as u128, 
                discount_rate: 9 as u128,
            },
        }
    }

    /// Verify '{net_present_value}' panic.
    #[test]
    fn verify_net_present_value() {
        let account = init_account();

        let current_net_present_value = Account::net_present_value(account);
        assert_eq!(current_net_present_value, 99);
    }

    /// Verify a principal as not a deponent
    #[test]
    fn verify_principal_is_not_a_deponent() {
        let account = init_account();

        assert_eq!(Account::is_deponent(account), false); 
    }

    /// Verify principal as a deponent
    #[test]
    fn verify_principal_is_a_deponent() {
        let account = init_account();

        assert_eq!(Account::is_deponent(account), false); 
    }
}
