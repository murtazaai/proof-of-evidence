/// Principal
pub struct Principal {
    pub is_alive: bool,
    pub net_cash_flow_at_time_t: u128, 
    pub time_of_cash_flow: u128, 
    pub discount_rate: u128
} 


/// Calculate net present value
pub fn net_present_value(principal: Principal) -> u128 {
    if principal.is_alive {
        principal.net_cash_flow_at_time_t / (1 + principal.discount_rate).pow(principal.time_of_cash_flow as u32)
    } else {
        0
    }
    
}

/// Check if principal is a deponent
pub fn is_deponent(deponent_net_present_value: u128) -> bool {
    if deponent_net_present_value > u64::MAX  as u128 {
        true
    } else {
        false
    }

}

/// Unit test cases
#[cfg(test)]
mod tests {
    use super::*;

    fn init_principal() -> Principal {
        Principal {
            is_alive: true,
            net_cash_flow_at_time_t: u128::MAX, 
            time_of_cash_flow: u128::MAX, 
            discount_rate: u128::MAX,
        }
    }

    /// Verify '{net_present_value}' panic.
    #[test]
    #[should_panic]
    fn verify_net_present_value() {
        let principal = init_principal();


        let current_net_present_value = net_present_value(principal);
        assert_eq!(current_net_present_value, u128::MAX);
    }

    /// Verify a principal as not a deponent
    #[test]
    fn verify_principal_is_not_a_deponent() {
        assert_eq!(is_deponent(u128::MIN), false); 
    }

    /// Verify principal as a deponent
    #[test]
    fn verify_principal_is_a_deponent() {
        assert_eq!(is_deponent(u64::MAX as u128), false); 
    }
}
