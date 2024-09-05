use super::BaseInfoService;

pub trait CompetitorsService<T: BaseInfoService> {
    fn competitors(&self) -> Vec<T>;
}
