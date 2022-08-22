#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]

mod uke_human_dns {

    use ink_storage::{traits::SpreadAllocate, Mapping};
    /// Emitted whenever a new user is registered.
    #[ink(event)]
    pub struct Register {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
    }

    /// Emitted whenever a username gets updated
    #[ink(event)]
    pub struct EditUsername {
        #[ink(topic)]
        old_name: Hash,
        #[ink(topic)]
        new_name: Hash,
        #[ink(topic)]
        from: AccountId,
    }

    /// Uke Human DNS ink! Smart Contract.  
    /// 
    /// Used to keep a registry of hash representations of human-readable usernames to AccountIds using ink!.
    /// 
    /// # Description
    ///
    /// Utilizes ink! to map cryptographic addresses to more human readable names, just like a DNS. 
    /// Coined the Human DNS, it essentially maps unique, human readable ids to otherwise illegible addresses via a hash.
    /// With this mapping of addresses, client-side implementations can then look up other users and add them to their contacts, 
    /// or write them a new message, or any other package of data in theory.

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct UkeHumanDns {
        /// User mapping of username hashes to accounts.
        username_to_id: Mapping<Hash, AccountId>,
        /// Default address of the contract.
        default_address: AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the name already exists upon registration.
        UsernameAlreadyExists,
        /// Returned if caller is not owner while required to.
        CallerIsNotOwner,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl UkeHumanDns {
        #[ink(constructor)]

        /// Creates a new human dns contract
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.default_address = Default::default();
            })
        }

        /// Retrieves a username from an account id from the global mapping.
        #[ink(message)]
        pub fn get_address(&self, name: Hash) -> AccountId {
            self.get_address_or_default(name)
        }

        /// Register a new username to the mapping.
        #[ink(message)]
        pub fn register(&mut self, name: Hash) -> Result<()> {
            let caller = self.env().caller();
            if self.username_to_id.contains(&name) {
                return Err(Error::UsernameAlreadyExists);
            }
            self.username_to_id.insert(&name, &caller);

            self.env().emit_event(Register { name, from: caller });

            Ok(())
        }

        /// Edit an existing username.
        #[ink(message)]
        pub fn edit_username(&mut self, old_name: Hash, new_name: Hash) -> Result<()> {
            let caller = self.env().caller();

            if self.get_address_or_default(old_name) != caller {
                return Err(Error::CallerIsNotOwner);
            }

            self.username_to_id.remove(&old_name);
            self.username_to_id.insert(&new_name, &caller);

            self.env().emit_event(EditUsername {
                old_name,
                new_name,
                from: caller,
            });

            Ok(())
        }

        /// Returns the address given the hash or the default address.
        fn get_address_or_default(&self, name: Hash) -> AccountId {
            self.username_to_id
                .get(&name)
                .unwrap_or(self.default_address)
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        use ink_lang as ink;

        fn set_next_caller(caller: AccountId) {
            ink_env::test::set_caller::<Environment>(caller);
        }

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<Environment>()
        }

        #[ink::test]
        fn default_works() {
            let uke_human_dns = UkeHumanDns::new();
            assert_eq!(uke_human_dns.default_address, Default::default());
        }

        #[ink::test]
        fn register_works() {
            let default_accounts = default_accounts();
            let name = Hash::from([0x01; 32]);

            set_next_caller(default_accounts.alice);
            let mut contract = UkeHumanDns::new();

            assert_eq!(contract.register(name), Ok(()));
            assert_eq!(contract.register(name), Err(Error::UsernameAlreadyExists));
        }

        #[ink::test]
        fn edit_works() {
            let default_accounts = default_accounts();
            let old_name = Hash::from([0x01; 32]);
            let new_name = Hash::from([0x02; 32]);

            set_next_caller(default_accounts.alice);
            let mut contract = UkeHumanDns::new();
            contract.register(old_name).unwrap();

            assert_eq!(contract.edit_username(old_name, new_name), Ok(()));
            assert_eq!(contract.get_address(new_name), default_accounts.alice);
        }

        #[ink::test]
        fn get_address_works() {
            let default_accounts = default_accounts();
            let name = Hash::from([0x01; 32]);

            set_next_caller(default_accounts.alice);
            let mut contract = UkeHumanDns::new();

            assert_eq!(contract.register(name), Ok(()));
            assert_eq!(contract.get_address(name), default_accounts.alice);
        }
    }
}
