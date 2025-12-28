use std::str::FromStr;

use scopes_rs::{hierarchy::Hierarchized, policy::{IntoPolicy, Policy}, scope::Scope};

#[derive(PartialEq)]
enum Scopes {
    User,
    UserSettings,
    UserProfile,
    UserProfileRead,
}

pub fn main() {
    let policy: Policy<Scopes> = Scopes::UserProfile.into_policy() & Scopes::UserSettings;

    println!("{}", policy.verify(&[Scopes::UserProfile]));
    println!("{}", policy.verify(&[Scopes::UserProfile, Scopes::UserSettings, Scopes::UserProfileRead]));
    println!("{}", policy.verify(&[Scopes::User]));

    let policy: Policy<Scopes> = Scopes::UserProfileRead.into();

    println!("{}", policy.verify(&[Scopes::UserProfile, Scopes::UserSettings]));
    println!("{}", policy.verify(&[Scopes::User]));

}

impl Hierarchized for Scopes {
    fn includes(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            (Scopes::User, _) => true,
            (Scopes::UserProfile, Scopes::UserProfileRead) => true,
            (_, _) => false
        }
    }
}

impl FromStr for Scopes {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Scope for Scopes {}
