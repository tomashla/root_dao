use scrypto::prelude::*;

blueprint! {
    struct Dao {

        dao_tokens_vault: Vault,
        staking_vault: Vault,
        voter_badges_vault:Vault,


    }

    impl Dao {

        pub fn new() -> ComponentAddress {
            let dao_tokens: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Root DAO")
                .metadata("symbol", "RDAO")
                .initial_supply(1_000_000);

            let voter_badges: Bucket = ResourceBuilder::new_non_fungible()
            .metadata("name","Badges")
            .metadata("symbol","BDG")
            .initial_supply(1_000_000);

            Self {
                dao_tokens_vault: Vault::with_bucket(dao_tokens),
                staking_vault: Vault::new(dao_tokens.resource_address()),
        voter_badges_vault:Vault::with_bucket(voter_badges)

            }
            .instantiate()
            .globalize()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token(&mut self) -> Bucket {
            info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.sample_vault.take(1)
        }
    }
}
