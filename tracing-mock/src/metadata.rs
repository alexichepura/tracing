use std::fmt;
use tracing_core::Metadata;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub(crate) struct ExpectedMetadata {
    pub(crate) name: Option<String>,
    pub(crate) level: Option<tracing::Level>,
    pub(crate) target: Option<String>,
}

<<<<<<< HEAD
impl ExpectedMetadata {
    pub(crate) fn check(
        &self,
        actual: &Metadata<'_>,
        ctx: fmt::Arguments<'_>,
        subscriber_name: &str,
    ) {
||||||| 386969ba
impl Expect {
    pub(in crate::support) fn check(&self, actual: &Metadata<'_>, ctx: fmt::Arguments<'_>) {
=======
impl ExpectedMetadata {
    pub(crate) fn check(
        &self,
        actual: &Metadata<'_>,
        ctx: fmt::Arguments<'_>,
        collector_name: &str,
    ) {
>>>>>>> origin/master
        if let Some(ref expected_name) = self.name {
            let name = actual.name();
            assert!(
                expected_name == name,
<<<<<<< HEAD
                "\n[{}] expected {} to be named `{}`, but got one named `{}`",
                subscriber_name,
||||||| 386969ba
                "expected {} to be named `{}`, but got one named `{}`",
=======
                "\n[{}] expected {} to be named `{}`, but got one named `{}`",
                collector_name,
>>>>>>> origin/master
                ctx,
                expected_name,
                name
            )
        }

        if let Some(ref expected_level) = self.level {
            let level = actual.level();
            assert!(
                expected_level == level,
<<<<<<< HEAD
                "\n[{}] expected {} to be at level `{:?}`, but it was at level `{:?}` instead",
                subscriber_name,
||||||| 386969ba
                "expected {} to be at level `{:?}`, but it was at level `{:?}` instead",
=======
                "\n[{}] expected {} to be at level `{:?}`, but it was at level `{:?}` instead",
                collector_name,
>>>>>>> origin/master
                ctx,
                expected_level,
                level,
            )
        }

        if let Some(ref expected_target) = self.target {
            let target = actual.target();
            assert!(
                expected_target == target,
<<<<<<< HEAD
                "\n[{}] expected {} to have target `{}`, but it had target `{}` instead",
                subscriber_name,
||||||| 386969ba
                "expected {} to have target `{}`, but it had target `{}` instead",
=======
                "\n[{}] expected {} to have target `{}`, but it had target `{}` instead",
                collector_name,
>>>>>>> origin/master
                ctx,
                expected_target,
                target,
            )
        }
    }
}

impl fmt::Display for ExpectedMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, " named `{}`", name)?;
        }

        if let Some(ref level) = self.level {
            write!(f, " at the `{:?}` level", level)?;
        }

        if let Some(ref target) = self.target {
            write!(f, " with target `{}`", target)?;
        }

        Ok(())
    }
}
