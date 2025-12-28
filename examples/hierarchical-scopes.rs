use std::{cmp::Ordering, str::FromStr};

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

impl Hierarchized for Scopes {}

impl PartialOrd for Scopes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal)
        }

        match (self, other) {
            (Scopes::User, _) => Some(Ordering::Greater),
            (_, Scopes::User) => Some(Ordering::Less),

            (Scopes::UserProfile, Scopes::UserProfileRead) => Some(Ordering::Greater),
            (Scopes::UserProfileRead, Scopes::UserProfile) => Some(Ordering::Less),

            (_, _) => None
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
