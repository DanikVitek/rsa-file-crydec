use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, ops::Deref};

use inquire::{
    validator::{CustomTypeValidator, ErrorMessage, Validation},
    CustomUserError,
};

macro_rules! impl_path {
    ($Ident:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $Ident(PathBuf);

        impl fmt::Display for $Ident {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0.display())
            }
        }

        impl FromStr for $Ident {
            type Err = anyhow::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(PathBuf::from_str(s)?))
            }
        }

        impl Deref for $Ident {
            type Target = PathBuf;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<Path> for $Ident {
            #[inline]
            fn as_ref(&self) -> &Path {
                self.0.as_path()
            }
        }

        impl DerefMut for $Ident {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

impl_path!(InputPath);
impl_path!(OutputPath);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathValidator;

impl CustomTypeValidator<InputPath> for PathValidator {
    #[inline]
    fn validate(&self, input: &InputPath) -> Result<Validation, CustomUserError> {
        if input.0.is_file() {
            return Ok(Validation::Valid);
        }

        Ok(Validation::Invalid(ErrorMessage::Custom(
            "Provided path is not a file".to_owned(),
        )))
    }
}

impl CustomTypeValidator<OutputPath> for PathValidator {
    #[inline]
    fn validate(&self, input: &OutputPath) -> Result<Validation, CustomUserError> {
        if !input.0.exists() || input.0.is_file() {
            return Ok(Validation::Valid);
        }

        Ok(Validation::Invalid(ErrorMessage::Custom(
            "Provided path exists and is not a file".to_owned(),
        )))
    }
}
