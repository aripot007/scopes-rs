use scopes_macros::Scope;

#[derive(PartialEq, Scope)]
#[scope(prefix = "https://api.ceten.fr/core/")]
enum Scopes {
    User,
    #[scope(rename = "settings")]
    UserSettings,
    UserProfile,
    UserProfileRead,
}

pub fn main() {

    let s = Scopes::User;

    let _b = matches!(s, Scopes::User | Scopes::UserProfile | Scopes::UserSettings);

    /*
    let policy: Policy<Scopes> = Scopes::UserProfile.into_policy() & Scopes::UserSettings;

    println!("{}", policy.verify(&[Scopes::UserProfile]));
    println!("{}", policy.verify(&[Scopes::UserProfile, Scopes::UserSettings, Scopes::UserProfileRead]));
    println!("{}", policy.verify(&[Scopes::User]));

    let policy: Policy<Scopes> = Scopes::UserProfileRead.into();

    println!("{}", policy.verify(&[Scopes::UserProfile, Scopes::UserSettings]));
    println!("{}", policy.verify(&[Scopes::User]));

    */
}
