#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod inkfit {

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Inkfit {
        users: Mapping<String, u32>,
        active_days: Vec<String>,
        admins: Vec<AccountId>
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum CustomError {
        UserDoesntExist,
    }

    impl Inkfit {
        #[ink(constructor)]
        pub fn default() -> Self {
            let caller = Self::env().caller();
            Self {
                users: Mapping::new(),
                active_days: Vec::new(),
                admins: vec![caller]
            }
        }

        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>) -> Self {
            Self {
                users: Mapping::new(),
                active_days: Vec::new(),
                admins: owners
            }
        }

        #[ink(message)]
        pub fn add_user(&mut self, user: String) {
            self.users.insert(user, &0);
        }

        #[ink(message)]
        pub fn get_user_activity_score(&self, user: String) -> Option<u32> {
            self.users.get(user)
        }

        #[ink(message)]
        pub fn add_activity(&mut self, user: String, activity: String, activity_date: String) {
            let mut active_user = self
                .users
                .get(&user)
                .ok_or(CustomError::UserDoesntExist)
                .unwrap();
            self.active_days
                .push(user.clone() + &activity_date + &activity);
            active_user += 1;
            self.users.insert(user, &active_user);
        }

        #[ink(message)]
        pub fn get_user_activities(&self, user: String) -> Option<Vec<String>> {
            let mut user_activities = Vec::new();
            for activity in &self.active_days {
                if activity.contains(&user) {
                    user_activities.push(activity.clone());
                }
            }
            Some(user_activities)
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
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Some(0));
            inkfit.add_activity(
                "pawel".to_owned(),
                "23 mins 3500 steps".to_string(),
                "26/03/2023".to_string(),
            );
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Some(1));
        }
    }
}
