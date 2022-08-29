use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pallidrome {
    // SETUP CONTRACT STATE
    all_pallindromes: Vec<String>,
}

#[near_bindgen]
impl Pallidrome {
    #[init]
    #[private]
    fn new() -> Self {
        let mut tmp = vec![];
        tmp.push("civic".to_string());
        tmp.push("radara".to_string());
        Pallidrome {
            all_pallindromes: tmp,
        }
    }

    fn get_sample(&self)->Vec<String>{
        return self.all_pallindromes.clone();
    }

    fn test_word(& mut self,phrase: String){
        let res = self.check_pallindome(phrase.clone());
        if res{
            self.all_pallindromes.push(phrase);
        }else{
            env::log_str("word is not pallindrome");
        }
    }

   
    fn check_pallindome(& self, phrase: String) -> bool {
        // pub fn is_palindrome(phrase: &str) -> bool {
        // get the chars iterator and associated index
        phrase
            .char_indices()
            .filter(|&(_, c)| c.is_alphabetic())
            // zip with the second half...
            .zip(
                phrase
                    .char_indices()
                    // which needs to be reversed...
                    .rev()
                    // and filter out bad cars
                    .filter(|&(_, c)| c.is_alphabetic()),
            )
            // accept all input until the indexes have crossed
            .take_while(|&((first_count, _), (last_count, _))| first_count < last_count)
            // check that all the chars from the begining and end match
            .all(|((_, first_char), (_, last_char))| {
                first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
            })
    }


    
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{ VMContextBuilder};
    use near_sdk::{ AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn test_word() {
        let mut app = Pallidrome::new();
        let _ = app.test_word("racecar".to_string());

        assert_eq!(app.all_pallindromes.len(), 3);
    }

    // TESTS HERE
    #[test]
    fn check_pallindome() {
        let  app = Pallidrome::new();
        let res = app.check_pallindome("racecar".to_string());

        assert_eq!(res, true);
    }
}
