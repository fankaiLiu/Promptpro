use rand::{Rng, seq::SliceRandom};

use crate::config::load_cofig::CFG;

pub struct MidJourney;

impl MidJourney {
    pub fn new() -> Self {
        Self
    }
    pub fn generate_prompts(&self, key_words: &str) -> String {
        let pick_one = &CFG.pick_one;
        let may_appear = &CFG.may_appear;
        let must_appear = &CFG.must_appear;
        let mut prompts = vec![];
        let mut rng = rand::thread_rng();

        for item in pick_one {
            let mut weight_sum = 0;
            for (_, weight) in item.iter() {
                weight_sum += weight;
            }
            let mut rand_num = rng.gen_range(0..weight_sum);
            for (string, weight) in item.iter() {
                if rand_num < *weight {
                    prompts.push(string.clone());
                    break;
                } else {
                    rand_num -= weight;
                }
            }
        }
        for (string, weight) in may_appear.iter() {
            let rand_num = rng.gen_range(0..=99);
            if rand_num < *weight {
                prompts.push(string.clone());
            }
        }
        prompts.extend(must_appear.iter().cloned());
        // Shuffle the prompts randomly
        prompts.shuffle(&mut rng);

        // Join the prompts into a comma-separated string
        let mut prompt_string = prompts.join(",");

        // Replace all occurrences of "0" with a space
        prompt_string = prompt_string.replace("0", " ");
        let res = key_words.to_string()+&prompt_string ;
        res
    }
}
