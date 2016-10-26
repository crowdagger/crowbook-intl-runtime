// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::RwLock;
use std::sync::RwLockReadGuard;

lazy_static! {
    pub static ref LANG: RwLock<String> = RwLock::new(String::from("en"));
}

/// Sets the lang
pub fn set_lang<S>(lang: S)
    where S: Into<String> {
    *LANG.write().unwrap() = lang.into();
}

/// Get the lang (or a guard on it)
///
/// This function should not be used directly
#[doc(hidden)]
pub fn __get_lang() -> RwLockReadGuard<'static, String> {
    LANG.read().unwrap()
}
