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
        admins: Vec<AccountId>,
        min_active_mins: u32,
        min_steps: u32,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CustomError {
        UserDoesNotExist,
        AccessOnlyForAdmins,
        AdminNotFound,
        TooLittleMins,
        TooLittleSteps,
    }

    pub type Result<T> = core::result::Result<T, CustomError>;

    impl Inkfit {
        #[ink(constructor)]
        pub fn default() -> Self {
            let caller = Self::env().caller();
            let mut admin_vec = Vec::new();
            admin_vec.push(caller);
            Self {
                users: Mapping::new(),
                active_days: Vec::new(),
                admins: admin_vec,
                min_active_mins: 0,
                min_steps: 0,
            }
        }

        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>, mins: u32, steps: u32) -> Self {
            Self {
                users: Mapping::new(),
                active_days: Vec::new(),
                admins: owners,
                min_active_mins: mins,
                min_steps: steps,
            }
        }

        #[ink(message)]
        pub fn add_user(&mut self, user: String) -> Result<()> {
            let caller = self.env().caller();
            if !self.admins.contains(&caller) {
                return Err(CustomError::AccessOnlyForAdmins);
            }
            self.users.insert(user, &0);
            Ok(())
        }

        #[ink(message)]
        pub fn add_admin(&mut self, admin: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.admins.contains(&caller) {
                return Err(CustomError::AccessOnlyForAdmins);
            }
            self.admins.push(admin);
            Ok(())
        }

        #[ink(message)]
        pub fn remove_admin(&mut self, admin: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.admins.contains(&caller) {
                return Err(CustomError::AccessOnlyForAdmins);
            }
            if !self.admins.contains(&admin) {
                return Err(CustomError::AdminNotFound);
            }
            let index_of_admin_to_remove = self.admins.iter().position(|&x| x == admin).unwrap();
            self.admins.swap_remove(index_of_admin_to_remove);
            Ok(())
        }

        #[ink(message)]
        pub fn get_user_activity_score(&self, user: String) -> Result<u32> {
            if !self.users.contains(&user) {
                return Err(CustomError::UserDoesNotExist);
            }
            Ok(self
                .users
                .get(user)
                .ok_or(CustomError::UserDoesNotExist)
                .unwrap())
        }

        #[ink(message)]
        pub fn add_activity(
            &mut self,
            user: String,
            active_mins: u32,
            steps_made: u32,
            activity_date: String,
        ) -> Result<()> {
            let caller = self.env().caller();
            if !self.admins.contains(&caller) {
                return Err(CustomError::AccessOnlyForAdmins);
            }

            if active_mins < self.min_active_mins {
                return Err(CustomError::TooLittleMins);
            }

            if steps_made < self.min_steps {
                return Err(CustomError::TooLittleSteps);
            }

            let mins_str = &active_mins.to_string();
            let steps_str = &steps_made.to_string();

            let activity = String::from(
                user.clone()
                    + " active mins: "
                    + mins_str
                    + " steps made: "
                    + steps_str
                    + " from: "
                    + &activity_date,
            );

            if !self.users.contains(&user) {
                return Err(CustomError::UserDoesNotExist);
            }

            let mut active_user = self
                .users
                .get(&user)
                .ok_or(CustomError::UserDoesNotExist)
                .unwrap();
            self.active_days.push(activity);
            active_user += 1;
            self.users.insert(user, &active_user);
            Ok(())
        }

        #[ink(message)]
        pub fn get_user_activities(&self, user: String) -> Result<Vec<String>> {
            if !self.users.contains(&user) {
                return Err(CustomError::UserDoesNotExist);
            }
            let mut user_activities = Vec::new();
            for activity in &self.active_days {
                if activity.contains(&user) {
                    user_activities.push(activity.clone());
                }
            }
            Ok(user_activities)
        }

        #[ink(message)]
        pub fn set_min_active_mins(&mut self, mins: u32) -> Result<()> {
            let caller = self.env().caller();
            if !self.admins.contains(&caller) {
                return Err(CustomError::AccessOnlyForAdmins);
            }
            self.min_active_mins = mins;
            Ok(())
        }

        #[ink(message)]
        pub fn set_min_steps(&mut self, steps: u32) -> Result<()> {
            let caller = self.env().caller();
            if !self.admins.contains(&caller) {
                return Err(CustomError::AccessOnlyForAdmins);
            }
            self.min_steps = steps;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn default_works() {
            let mut inkfit = Inkfit::default();
            let user_to_add = "pawel".to_string();
            assert_eq!(inkfit.add_user(user_to_add), Ok(()));
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Ok(0));
            assert_eq!(
                inkfit.add_activity("pawel".to_owned(), 23, 4000, "26/03/2023".to_string()),
                Ok(())
            );
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Ok(1));
        }

        #[ink::test]
        fn min_mins_work() {
            let mut inkfit = Inkfit::default();
            let user_to_add = "pawel".to_string();
            assert_eq!(inkfit.add_user(user_to_add), Ok(()));
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Ok(0));
            assert_eq!(inkfit.set_min_active_mins(40), Ok(()));
            assert_eq!(
                inkfit.add_activity("pawel".to_owned(), 23, 4000, "26/03/2023".to_string()),
                Err(CustomError::TooLittleMins)
            );
            assert_eq!(
                inkfit.add_activity("pawel".to_owned(), 43, 4000, "26/03/2023".to_string()),
                Ok(())
            );
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Ok(1));
        }

        #[ink::test]
        fn min_steps_work() {
            let mut inkfit = Inkfit::default();
            let user_to_add = "pawel".to_string();
            assert_eq!(inkfit.add_user(user_to_add), Ok(()));
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Ok(0));
            assert_eq!(inkfit.set_min_steps(8000), Ok(()));
            assert_eq!(
                inkfit.add_activity("pawel".to_owned(), 23, 4000, "26/03/2023".to_string()),
                Err(CustomError::TooLittleSteps)
            );
            assert_eq!(
                inkfit.add_activity("pawel".to_owned(), 43, 10000, "26/03/2023".to_string()),
                Ok(())
            );
            assert_eq!(inkfit.get_user_activity_score("pawel".to_string()), Ok(1));
        }

        #[ink::test]
        fn only_extisitng_users_work() {
            let mut inkfit = Inkfit::default();
            let user_to_add = "pawel".to_string();
            assert_eq!(inkfit.add_user(user_to_add), Ok(()));
            assert_eq!(
                inkfit.add_activity("krzysiek".to_owned(), 23, 4000, "26/03/2023".to_string()),
                Err(CustomError::UserDoesNotExist)
            );
            assert_eq!(
                inkfit.get_user_activity_score("krzysiek".to_string()),
                Err(CustomError::UserDoesNotExist)
            );
            assert_eq!(
                inkfit.get_user_activities("krzysiek".to_string()),
                Err(CustomError::UserDoesNotExist)
            );
        }

        #[ink::test]
        fn only_admins_can_add_new_admins() {
            let mut inkfit = Inkfit::default();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(inkfit.add_admin(accounts.alice), Err(CustomError::AccessOnlyForAdmins));
        }
    }
}
