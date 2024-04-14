use solana_program::account_info::AccountInfo;

pub fn is_account_initialized(
    account: &AccountInfo
) -> bool {
    if account.lamports() == 0 {
        return false;
    }
    if account.data_is_empty() {
        return false;
    }
    true
}