#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// Define storage keys to keep track of contract data on the blockchain
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,             // Stores the organizer's Address
    EventName,         // Stores the name of the event
    Certificate(Address), // Maps an attendee's Address to their certificate status
}

#[contract]
pub struct EventCertificateContract;

#[contractimpl]
impl EventCertificateContract {
    
    /// Initializes the contract with an admin and the event name.
    /// Can only be called once.
    pub fn initialize(env: Env, admin: Address, event_name: String) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract is already initialized!");
        }
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::EventName, &event_name);
    }

    /// Allocates/Issues a certificate to an attendee.
    /// Only the Admin can call this function.
    pub fn issue(env: Env, to: Address) {
        // 1. Fetch the admin from storage and verify their signature
        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .expect("Contract not initialized");
        admin.require_auth();

        // 2. Create the storage key for this specific attendee
        let key = DataKey::Certificate(to.clone());
        
        // 3. Ensure they don't already have a certificate for this event
        if env.storage().persistent().has(&key) {
            panic!("A certificate has already been issued to this address.");
        }

        // 4. Save the certificate to persistent storage (true = certificate exists)
        env.storage().persistent().set(&key, &true);
    }

    /// Public function to check if an address holds a certificate for this event.
    pub fn has_certificate(env: Env, attendee: Address) -> bool {
        let key = DataKey::Certificate(attendee);
        env.storage().persistent().get(&key).unwrap_or(false)
    }

    /// Public function to fetch the event name.
    pub fn get_event_name(env: Env) -> String {
        env.storage().instance().get(&DataKey::EventName).expect("Contract not initialized")
    }
}