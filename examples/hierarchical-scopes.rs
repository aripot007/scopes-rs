use scopes_macros::Scope;
use scopes_rs::policy::{IntoPolicy, Policy};

#[cfg(not(feature = "hierarchy"))]
compile_error!("This example requires the `hierarchy` feature");

#[derive(PartialEq, Scope)]
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
    println!("{}", policy.verify(&[&scope_user]));

    let policy: Policy<Scopes> = Scopes::UserProfileRead.into();

    println!("{}", policy.verify(&[&Scopes::UserProfile, &Scopes::UserSettings]));
    println!("{}", policy.verify(&[&scope_user]));
}
