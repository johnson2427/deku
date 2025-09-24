#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum JobStatus<AccountId> {
        Pending,
        Claimed(AccountId),
        Completed,
    }

    #[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Job<AccountId> {
        pub id: u64,
        pub creator: AccountId,
        pub manifest: Vec<u8>,
        pub status: JobStatus<AccountId>,
    }

    #[pallet::storage]
    pub type Jobs<T: Config> = StorageMap<_, Blake2_128Concat, u64, Job<T::AccountId>, OptionQuery>;

    #[pallet::storage]
    pub type JobCounter<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        JobSubmitted(T::AccountId, Vec<u8>),
        JobClaimed(u64, T::AccountId),
        JobCompleted(T::AccountId, Vec<u8>),
    }

    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn submit_job(origin: T::AccountId, manifest: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let job_id = JobCounter::<T>::get();
            let job = Job {
                id: job_id,
                creator: who.clone(),
                manifest,
                status: JobStatus::Pending,
            };
            Jobs::<T>::insert(job_id, job);
            JobCounter::<T>::put(job_id + 1);
            Self::deposit_event(Event::JobSubmitted(origin, manifest));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn claim_job(job_id: u64, origin: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let job_id = JobCounter::<T>::get();
            let job = Job {
                id: job_id,
                creator: who.clone(),
                manifest,
                status: JobStatus::Pending,
            }
            Jobs::<T>::insert(job_id, job);
            JobCounter::<T>::put(job_id + 1);
            Self::deposit_event(Event::JobSubmitted(job_id))
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn complete_job(job_id: u64, origin: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Jobs::<T>::try_mutate(job_id, |job_opt| -> DispatchResult {
                let job = job_opt.as_mut().ok_or(Error::<T>::JobNotFound)?;
                ensure!(matches!(job.status, JobStatus::Claimed(ref claimer) if claimer == &who), Error::<T>::NoJobClaimer);
                job.status = JobStatus::Completed;
                Ok(())
            })?;
            Self::deposit_event(Event::JobCompleted(job_id, who));
            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        JobNotFound,
        JobAlreadyClaimed,
        NotJobClaimer,
    }
}
