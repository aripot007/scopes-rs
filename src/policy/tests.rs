use std::str::FromStr;

#[cfg(feature = "hierarchy")]
use crate::hierarchy::Hierarchized;

use crate::{policy::{IntoPolicy, Policy}, scope::Scope};

#[derive(Debug, Clone, PartialEq)]
enum MyScope {
    Foo,
    FooBar,
    Bar,
}

// region:    Boilerplate scope implementation

impl FromStr for MyScope {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "foo" => Ok(MyScope::Foo),
            "foo.bar" => Ok(MyScope::FooBar),
            "bar" => Ok(MyScope::Bar),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "hierarchy")]
impl Hierarchized for MyScope {
    fn includes(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }
        match self {
            MyScope::Foo => other == &MyScope::FooBar,
            _ => false,
        }
    }
}

impl Scope for MyScope {}

// endregion: Boilerplate scope implementation

#[test]
fn test_single_scope_policy() {
    let policy = Policy::Scope(MyScope::Foo);

    MyScope::Foo.into_policy();

    assert_eq!(policy, MyScope::Foo.into_policy());

    assert!(policy.verify(&[&MyScope::Foo]));
    assert!(!policy.verify(&[&MyScope::FooBar]));
}

#[test]
fn test_and_policy() {
    let policy = Policy::Scope(MyScope::Foo) & Policy::Scope(MyScope::Bar);

    assert_eq!(policy, Policy::AllOf(vec![Policy::Scope(MyScope::Foo), Policy::Scope(MyScope::Bar)]));

    assert!(policy.verify(&[&MyScope::Foo, &MyScope::Bar]));

    assert!(!policy.verify(&[&MyScope::Foo]));
    assert!(!policy.verify(&[&MyScope::Bar]));
}

#[test]
fn test_and_reduction() {
    
    let double_allof = Policy::AllOf(vec![MyScope::Foo.into_policy()]) & Policy::AllOf(vec![MyScope::FooBar.into_policy(), MyScope::Bar.into_policy()]);
    
    let lhs_allof = Policy::AllOf(vec![MyScope::Foo.into_policy(), MyScope::FooBar.into_policy()]) & MyScope::Bar;
    
    let rhs_allof = MyScope::Foo.into_policy() & Policy::AllOf(vec![MyScope::FooBar.into_policy(), MyScope::Bar.into_policy()]);


    let double_allof_vec = match double_allof {
        Policy::AllOf(v) => v,
        _ => panic!("Expected Policy::AllOf variant, got {:?}", double_allof)
    };
    assert_eq!(1, double_allof_vec.iter().filter(|a| a.eq(&&MyScope::Foo.into_policy())).count());
    assert_eq!(1, double_allof_vec.iter().filter(|a| a.eq(&&MyScope::FooBar.into_policy())).count());
    assert_eq!(1, double_allof_vec.iter().filter(|a| a.eq(&&MyScope::Bar.into_policy())).count());


    let lhs_allof_vec = match lhs_allof {
        Policy::AllOf(v) => v,
        _ => panic!("Expected Policy::AllOf variant, got {:?}", lhs_allof)
    };
    assert_eq!(1, lhs_allof_vec.iter().filter(|a| a.eq(&&MyScope::Foo.into_policy())).count());
    assert_eq!(1, lhs_allof_vec.iter().filter(|a| a.eq(&&MyScope::FooBar.into_policy())).count());
    assert_eq!(1, lhs_allof_vec.iter().filter(|a| a.eq(&&MyScope::Bar.into_policy())).count());


    let rhs_allof_vec = match rhs_allof {
        Policy::AllOf(v) => v,
        _ => panic!("Expected Policy::AllOf variant, got {:?}", rhs_allof)
    };
    assert_eq!(1, rhs_allof_vec.iter().filter(|a| a.eq(&&MyScope::Foo.into_policy())).count());
    assert_eq!(1, rhs_allof_vec.iter().filter(|a| a.eq(&&MyScope::FooBar.into_policy())).count());
    assert_eq!(1, rhs_allof_vec.iter().filter(|a| a.eq(&&MyScope::Bar.into_policy())).count());

    assert_eq!(Policy::<MyScope>::DenyAll, Policy::<MyScope>::DenyAll & MyScope::Foo.into_policy());
    assert_eq!(Policy::<MyScope>::DenyAll, MyScope::Foo.into_policy() & Policy::<MyScope>::DenyAll);

}

#[test]
fn test_or_reduction() {
    
    let double_oneof = Policy::OneOf(vec![MyScope::Foo.into_policy()]) | Policy::OneOf(vec![MyScope::FooBar.into_policy(), MyScope::Bar.into_policy()]);
    
    let lhs_oneof = Policy::OneOf(vec![MyScope::Foo.into_policy(), MyScope::FooBar.into_policy()]) | MyScope::Bar;
    
    let rhs_oneof = MyScope::Foo.into_policy() | Policy::OneOf(vec![MyScope::FooBar.into_policy(), MyScope::Bar.into_policy()]);


    let double_oneof_vec = match double_oneof {
        Policy::OneOf(v) => v,
        _ => panic!("Expected Policy::OneOf variant, got {:?}", double_oneof)
    };
    assert_eq!(1, double_oneof_vec.iter().filter(|a| a.eq(&&MyScope::Foo.into_policy())).count());
    assert_eq!(1, double_oneof_vec.iter().filter(|a| a.eq(&&MyScope::FooBar.into_policy())).count());
    assert_eq!(1, double_oneof_vec.iter().filter(|a| a.eq(&&MyScope::Bar.into_policy())).count());


    let lhs_oneof_vec = match lhs_oneof {
        Policy::OneOf(v) => v,
        _ => panic!("Expected Policy::OneOf variant, got {:?}", lhs_oneof)
    };
    assert_eq!(1, lhs_oneof_vec.iter().filter(|a| a.eq(&&MyScope::Foo.into_policy())).count());
    assert_eq!(1, lhs_oneof_vec.iter().filter(|a| a.eq(&&MyScope::FooBar.into_policy())).count());
    assert_eq!(1, lhs_oneof_vec.iter().filter(|a| a.eq(&&MyScope::Bar.into_policy())).count());


    let rhs_oneof_vec = match rhs_oneof {
        Policy::OneOf(v) => v,
        _ => panic!("Expected Policy::OneOf variant, got {:?}", rhs_oneof)
    };
    assert_eq!(1, rhs_oneof_vec.iter().filter(|a| a.eq(&&MyScope::Foo.into_policy())).count());
    assert_eq!(1, rhs_oneof_vec.iter().filter(|a| a.eq(&&MyScope::FooBar.into_policy())).count());
    assert_eq!(1, rhs_oneof_vec.iter().filter(|a| a.eq(&&MyScope::Bar.into_policy())).count());


    assert_eq!(Policy::<MyScope>::AllowAll, Policy::<MyScope>::AllowAll | MyScope::Foo.into_policy());
    assert_eq!(Policy::<MyScope>::AllowAll, MyScope::Foo.into_policy() | Policy::<MyScope>::AllowAll);
}

#[test]
fn test_or_policy() {
    let policy = Policy::Scope(MyScope::Foo) | Policy::Scope(MyScope::Bar);

    assert_eq!(policy, Policy::OneOf(vec![Policy::Scope(MyScope::Foo), Policy::Scope(MyScope::Bar)]));

    assert!(policy.verify(&[&MyScope::Foo, &MyScope::Bar]));
    assert!(policy.verify(&[&MyScope::Foo]));
    assert!(policy.verify(&[&MyScope::Bar]));

    assert!(!policy.verify(&[&MyScope::FooBar]));
}

#[test]
fn test_not_policy() {
    let policy = !Policy::Scope(MyScope::Foo);

    assert_eq!(policy, Policy::Not(Box::new(Policy::Scope(MyScope::Foo))));
    assert_eq!(!policy.clone(), Policy::Scope(MyScope::Foo));

    assert!(!policy.verify(&[&MyScope::Foo]));
    assert!(!policy.verify(&[&MyScope::Bar, &MyScope::Foo]));
    
    assert!(policy.verify(&[&MyScope::Bar]));
    assert!(policy.verify(&[]));
}

#[test]
fn test_allow_deny_policy() {
    assert!(Policy::<MyScope>::AllowAll.verify(&[]));
    assert!(Policy::<MyScope>::AllowAll.verify(&[&MyScope::Foo, &MyScope::FooBar, &MyScope::Bar]));

    assert!(!Policy::<MyScope>::DenyAll.verify(&[]));
    assert!(!Policy::<MyScope>::DenyAll.verify(&[&MyScope::Foo, &MyScope::FooBar, &MyScope::Bar]));

    assert_eq!(!Policy::<MyScope>::AllowAll, Policy::<MyScope>::DenyAll);
    assert_eq!(!Policy::<MyScope>::DenyAll, Policy::<MyScope>::AllowAll);
}

#[test]
fn test_hierarchy() {
    let policy = MyScope::FooBar.into_policy();

    #[cfg(feature = "hierarchy")]
    assert_eq!(true, policy.verify(&[&MyScope::Foo]));

    #[cfg(not(feature = "hierarchy"))]
    assert_eq!(false, policy.verify(&[&MyScope::Foo]));
}
