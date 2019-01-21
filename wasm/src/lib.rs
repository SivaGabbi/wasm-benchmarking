#![no_std]
#![allow(non_snake_case)]
#![feature(use_extern_macros)]
#![feature(proc_macro_gen)]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;

pub mod token {
    use pwasm_ethereum;
    use pwasm_abi::types::*;

    // eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    // #[eth_abi(Endpoint2, Client2)]
    /// trait Contract2 { }
    /// ```
    ///
    /// Creates an endpoint implementation named `Endpoint2` and a
    /// client implementation named `Client2` for the interface
    /// defined in the `Contract2` trait.
    // #[proc_macro_attribute]
    // pub fn eth_abi(
    //    args: proc_macro::TokenStream, --> these here must be endpoint2 and client2 mentioned above 
    //    input: proc_macro::TokenStream, --> this must be the contract 2 trait above which eth_abi is mentioned 
    // ) -> proc_macro::TokenStream {
    //     let args_toks = parse_macro_input!(args as syn::AttributeArgs);
    //     let input_toks = parse_macro_input!(input as syn::Item);

    //     let output = match impl_eth_abi(args_toks, input_toks) { --> impl_eth_abi is another function in the code which is executed.. 
    //         Ok(output) => output,
    //         Err(err) => panic!("[eth_abi] encountered error: {}", err),
    //     };

    //     output.into()
    // }
    use pwasm_abi_derive::eth_abi;
    // https://github.com/paritytech/pwasm-abi/blob/master/derive/src/lib.rs

    static TOTAL_SUPPLY_KEY: H32 = H32([2]);
    static OWNER_KEY: H256 = H256([3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenInterface {
        /// The constructor
        fn constructor(&mut self, _total_supply: H32);
        /// Total amount of tokens
        #[constant]
        fn totalSupply(&mut self) -> H32;
        /// What is the balance of a particular account?
        #[constant]
        fn balanceOf(&mut self, _owner: Address) -> H32;
        /// Transfer the balance from owner's account to another account
        fn transfer(&mut self, _to: Address, _amount: H32) -> bool;
        /// Event declaration
        #[event]
        fn Transfer(&mut self, indexed_from: Address, indexed_to: Address, _value: U256);
    }

    pub struct TokenContract;

    impl TokenInterface for TokenContract {
        fn constructor(&mut self, total_supply: U256) {
            let sender = pwasm_ethereum::sender();
            // Set up the total supply for the token
            pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
            // Give all tokens to the contract owner
            pwasm_ethereum::write(&balance_key(&sender), &total_supply.into());
            // Set the contract owner
            pwasm_ethereum::write(&OWNER_KEY, &H256::from(sender).into());
        }

        fn totalSupply(&mut self) -> U256 {
            U256::from_big_endian(&pwasm_ethereum::read(&TOTAL_SUPPLY_KEY))
        }

        fn balanceOf(&mut self, owner: Address) -> U256 {
            read_balance_of(&owner)
        }

        fn transfer(&mut self, to: Address, amount: U256) -> bool {
            let sender = pwasm_ethereum::sender();
            let senderBalance = read_balance_of(&sender);
            let recipientBalance = read_balance_of(&to);
            if amount == 0.into() || senderBalance < amount || to == sender {
                false
            } else {
                let new_sender_balance = senderBalance - amount;
                let new_recipient_balance = recipientBalance + amount;
                pwasm_ethereum::write(&balance_key(&sender), &new_sender_balance.into());
                pwasm_ethereum::write(&balance_key(&to), &new_recipient_balance.into());
                self.Transfer(sender, to, amount);
                true
            }
        }
    }

    // Reads balance by address
    fn read_balance_of(owner: &Address) -> U256 {
        U256::from_big_endian(&pwasm_ethereum::read(&balance_key(owner)))
    }

    // Generates a balance key for some address.
    // Used to map balances with their owners.
    fn balance_key(address: &Address) -> H256 {
        let mut key = H256::from(address);
        key[0] = 1; // just a naiive "namespace";
        key
    }
}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

