use super::*;

pub struct Container {
    concurrency_primitives_system_wide_singleton: concurrency_primitives::SystemWideSingleton,
    arch_info: arch::Arch,
}

impl Container {
    pub fn build() -> Self {
        Self {
            concurrency_primitives_system_wide_singleton: quickstart_template::concurrency_primitives::SystemWideSingleton::new(),
            arch_info: quickstart_template::arch::Arch::new(),
        }
    }

    pub fn resolve_ref_concurrency_primitives_singleton(&self) -> &concurrency_primitives::SystemWideSingleton {
        &self.concurrency_primitives_system_wide_singleton
    }

    pub fn resolve_ref_arch_info(&self) -> &arch::Arch {
        &self.arch_info
    }
}
