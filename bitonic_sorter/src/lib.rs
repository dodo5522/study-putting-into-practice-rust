pub mod first;
pub mod second;

#[derive(PartialEq, Eq)]
pub enum SortOrder {
  Ascending,
  Descending,
}
