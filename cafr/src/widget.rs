pub struct Target<T> 
where
    T: Send + std::fmt::Debug
{
    msg: T,
    widget: Box<dyn Widget>
}

pub trait Widget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

}