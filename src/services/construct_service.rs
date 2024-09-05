pub trait ConstructService {
    fn construct(id: String, name: String) -> Self;
}
