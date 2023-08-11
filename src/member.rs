use scrypto::prelude::*;

// data within membership cards, can get updated
#[derive(ScryptoSbor, NonFungibleData)]
struct MembershipData{
    // user level, great for gamification and advancing to the next level
    level: String
}

// define task and reward
#[derive(ScryptoSbor)]
pub enum Tasks{
    Vote(String, u32),
    AttendEvent(String, u32),
    SayHi(u32)
}

#[blueprint]
mod member {
    struct Member {
        // rewards token
        rewards_token_resource_manager: ResourceManager,
        // where member's card is held
        member_card_resource_manager: ResourceManager,
    }
    impl Member {

        // Call instantiate_member to create a new member (will have 1 membership card and 0 rewards)
        pub fn instantiate_member() -> Global<Member> {

            let rewards_token_resource_manager: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(DIVISIBILITY_NONE)
            .metadata(metadata!(
                init {
                    "name" => "Reward_Token".to_owned(), locked;
                    "symbol" => "REW".to_owned(), locked;
                    "description" => "Rewards for activity".to_owned(), locked;
                }
            ))
            .create_with_no_initial_supply();

            // Create resource representing membership card, to be minted by user. Maximum 1 can be minted per account.
            let member_card_resource_manager: ResourceManager = ResourceBuilder::new_ruid_non_fungible::<MembershipData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Member Card", locked;
                        "symbol" => "MEM_CARD", locked;
                    }
                }).create_with_no_initial_supply();

            // Instantiate a Member component with member card and empty rewards bucket
            Self {
                rewards_token_resource_manager,
                member_card_resource_manager,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        /**
         * Only an account without an existing membership card can mint a new one
         */
        pub fn mint_member_card(&self) -> Bucket {
            // if account has a user badge then error out
            
            // mint and return user badge
            let member_card = self.member_card_resource_manager.mint(1);

            member_card
        }

        /**
         * Complete task and get the associated award
         */
        pub fn get_reward_for_task(&self, task: Tasks) -> Bucket{
            let rewards: Bucket;
            match task {
                Tasks::Vote(poll_name, reward_name) => {
                    println!("Member voted for poll: {}, collect {} pts.", poll_name, reward_name);
                    rewards = self.rewards_token_resource_manager.mint(reward_name);
                }
                Tasks::AttendEvent(event_name, reward_name) => {
                    println!("Member attended event: {}, collect {} pts.", event_name, reward_name);
                    rewards = self.rewards_token_resource_manager.mint(reward_name);
                }
                Tasks::SayHi(reward_name) => {
                    println!("Member said hi!, collect {} pts.", reward_name);
                    rewards = self.rewards_token_resource_manager.mint(reward_name)
                }
            }

            rewards
        }
    }
}