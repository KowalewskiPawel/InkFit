#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod inkfit {

    use ink::storage::{Mapping};
#[ink(storage)]
    pub struct Inkfit {
        users: Mapping<String, u32>,
        active_days: Mapping<String, String>
    }

    impl Inkfit {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { users: Mapping::new(), active_days: Mapping::new() }
        }

        #[ink(message)]
        pub fn add_user(&mut self, user: String) {
            self.users.insert(user, &0);
        }

        #[ink(message)]
        pub fn get_user_activites(&self, user: String) -> Option<u32> {
            self.users.get(user)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn default_works() {
            let mut inkfit = Inkfit::default();
            let user_to_add = "pawel".to_string();
            inkfit.add_user(user_to_add);
            assert_eq!(inkfit.get_user_activites("pawel".to_string()), Some(0));
        }
    }
}
