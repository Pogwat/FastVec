use crate::shared::Errors;
#[cfg(feature = "FastRemove")] use crate::absolute::AbsoluteErrors; 

#[derive(Debug)]
pub enum FullError {
    FastVec(Errors),
    #[cfg(feature = "FastRemove")]  Absolute(AbsoluteErrors),
}

impl From<Errors> for FullError {
    fn from(err: Errors) -> Self {
        FullError::FastVec(err)
    }
}

#[cfg(feature = "FastRemove")]
impl From<AbsoluteErrors> for FullError {
    fn from(err: AbsoluteErrors) -> Self {
        FullError::Absolute(err)
    }
}