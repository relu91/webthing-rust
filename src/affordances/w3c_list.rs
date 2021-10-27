

#[derive(Debug,Clone)]
pub enum W3CList<T>  {
    Single(T),
    List(Vec<T>),
    None
}





