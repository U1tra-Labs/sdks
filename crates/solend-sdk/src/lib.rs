pub mod transaction;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

    use super::*;

    #[test]
    fn it_works() {
        let result = transaction::get_size_of_transaction(
            vec![Instruction {
                program_id: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
                accounts: vec![],
                data: vec![]
            }],
            false,
            None
        ).unwrap();
        assert_eq!(result, 4);
    }
}
