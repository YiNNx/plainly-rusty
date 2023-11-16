#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::comments")]
#[seaography(entity = "crate::entities::posts")]
#[seaography(entity = "crate::entities::posttags")]
#[seaography(entity = "crate::entities::tags")]
#[seaography(entity = "crate::entities::users")]
pub struct QueryRoot;
