use super::*;
use std::any::TypeId;

#[test]
fn resolve_concurrency_primitives_singleton_yields_concurrency_primitives_system_wide_singleton {
    // given
    let container = container::Container::build();

    // when
    let result = container.resolve_ref_concurrency_primitives_singleton();

    // then
    assert_eq!(TypeId::of::)
}

quickstart_template::concurrency_primitives::SystemWideSingleton::new(),
arch_info: quickstart_template::arch::Arch::new(),
