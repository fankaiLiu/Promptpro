// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

#![deny(unsafe_code)]

use config::load_cofig::CFG;
use generate_prompts::mid_journey::MidJourney;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();
pub mod config;
pub mod generate_prompts;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();
    let app = App::new().unwrap();
    let app_weak = app.as_weak();

    app.global::<GeneratePageAdapter>().on_generate(move || {
        let mj=MidJourney::new();
        let key_words=app_weak
        .unwrap()
        .global::<GeneratePageAdapter>()
        .get_key_words().to_string();
        let mut prompts = String::new();
        for _ in 0..10 {
            prompts.push_str(&mj.generate_prompts(&key_words));
            prompts.push_str("\n");
        }

        app_weak
            .unwrap()
            .global::<GeneratePageAdapter>()
            .set_prompts(prompts.into());
    });
    app.run().unwrap();
}
