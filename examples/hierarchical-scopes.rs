#![cfg_attr(all(test, not(feature = "hierarchy")), ignore("This example requires the `hierarchy` feature"))]

use scopes_macros::Scope;
use scopes_rs::policy::{IntoPolicy, Policy};

#[derive(Clone, PartialEq, Scope)]
enum Scopes {
    User,
    UserSettings,
    UserProfile,
    UserProfileRead,
}

pub fn main() {

    let scope_user = str::parse::<Scopes>("user").unwrap();
    let scope_user_profile = str::parse::<Scopes>("user.profile").unwrap();
    let scope_user_settings = str::parse::<Scopes>("user.settings").unwrap();

    let policy: Policy<Scopes> = Scopes::UserProfile.into_policy() & Scopes::UserSettings;

    println!("{}", policy.verify(&[&scope_user_profile]));
    println!("{}", policy.verify(&[&scope_user_settings, &scope_user_profile]));
    println!("{}", policy.verify(&[&scope_user.clone()]));

    let policy: Policy<Scopes> = Scopes::UserProfileRead.into();

    println!("{}", policy.verify(&[Scopes::UserProfile, Scopes::UserSettings]));
    println!("{}", policy.verify(&[&scope_user]));
}
