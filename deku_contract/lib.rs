#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod deku_contract {
    use scale::{Decode, Encode};
    use scale_info::prelude::vec::Vec;

    #[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Job {
        pub id: u64,
        pub creator: AccountId,
        pub manifest: Vec<u8>,
        pub status: u8,
    }

    #[ink(storage)]
    pub struct Deku {
        jobs: ink::storage::Mapping<u64, Job>,
        next_id: u64,
    }

    impl Deku {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                jobs: Default::default(),
                next_id: 0,
            }
        }

        #[ink(message)]
        pub fn submit_job(&mut self, manifest: Vec<u8>) -> u64 {
            let id = self.next_id;
            let job = Job {
                id,
                creator: self.env().caller(),
                manifest,
                status: 0,
            };
            self.jobs.insert(id, &job);
            self.next_id = self
                .next_id
                .checked_add(1)
                .expect("overflow on job ID increment");
            id
        }

        #[ink(message)]
        pub fn get_job(&self, id: u64) -> Option<Job> {
            self.jobs.get(id)
        }

        #[ink(message)]
        pub fn claim_job(&mut self, id: u64) -> Result<(), JobError> {
            let mut job = self.jobs.get(id).ok_or(JobError::NotFound)?;
            if job.status != 0 {
                return Err(JobError::NotClaimed);
            }
            if job.creator == self.env().caller() {
                return Err(JobError::NotAuthorized);
            }
            job.status = 1;
            self.jobs.insert(id, &job);
            Ok(())
        }

        #[ink(message)]
        pub fn complete_job(&mut self, id: u64) -> Result<(), JobError> {
            let mut job = self.jobs.get(id).ok_or(JobError::NotFound)?;
            if job.status != 1 {
                return Err(JobError::NotClaimed);
            }

            if job.creator != self.env().caller() {
                return Err(JobError::NotAuthorized);
            }
            job.status = 2;
            self.jobs.insert(id, &job);
            Ok(())
        }
    }

    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum JobError {
        NotFound,
        NotClaimed,
        NotAuthorized,
        IdOverflow,
    }
}
