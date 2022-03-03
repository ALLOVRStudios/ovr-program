mod ui_amount_to_amount_test {
    use allovr_token::utils::ui_amount_to_amount;
    use solana_program::msg;
    use solana_program_test::*;
    #[tokio::test]
    async fn test() {
        assert_eq!(ui_amount_to_amount(1.0), 1000000000);
        assert_eq!(ui_amount_to_amount(1.01), 1010000000);
        assert_eq!(ui_amount_to_amount(0.123456789), 123456789);
        assert_eq!(ui_amount_to_amount(0.0000000009), 0);
        assert_eq!(ui_amount_to_amount(0.000000007), 7);
        assert_eq!(ui_amount_to_amount(0.00000000711), 7);
    }
}
