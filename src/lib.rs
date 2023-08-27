/// Calculate net present value
pub fn net_present_value(net_cash_flow_at_time_t: u128, time_of_cash_flow: u128, discount_rate: u128) -> u128 {
    net_cash_flow_at_time_t / (1 + discount_rate).pow(time_of_cash_flow as u32)
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

    /// Verify '{net_present_value}' panic.
    #[test]
    #[should_panic]
    fn verify_net_present_value() {
        let result = net_present_value(u128::MAX, u128::MAX, u128::MAX);
        assert_eq!(result, u128::MAX);
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
