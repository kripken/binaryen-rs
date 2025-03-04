use crate::Module;
use std::os::raw::c_char;

/// Convert some random array of bytes to a Module.
pub fn translate_to_fuzz(seed: &[u8]) -> Module {
    if seed.len() == 0 {
        return Module::new();
    }

    unsafe {
        let raw_module =
            binaryen_sys::translateToFuzz(seed.as_ptr() as *const c_char, seed.len(), true);
        Module::from_raw(raw_module)
    }
}

/// Convert some random array of bytes to a WASM-MVP-only Module.
pub fn translate_to_fuzz_mvp(seed: &[u8]) -> Module {
    if seed.len() == 0 {
        return Module::new();
    }

    unsafe {
        let raw_module =
            binaryen_sys::translateToFuzz(seed.as_ptr() as *const c_char, seed.len(), false);
        Module::from_raw(raw_module)
    }
}

/// Mutate a Module based on some random array of bytes.
pub fn mutate_to_fuzz(module: &mut Module, seed: &[u8]) {
    if seed.len() == 0 {
        return;
    }

    unsafe {
        binaryen_sys::mutateToFuzz(module.inner.raw, seed.as_ptr() as *const c_char, seed.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::Module;
    use super::translate_to_fuzz;
    use super::translate_to_fuzz_mvp;
    use super::mutate_to_fuzz;
    use rand::{self, RngCore};

    #[test]
    fn test_translate_to_fuzz() {
        let mut seed = vec![0; 1000];
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut seed);
            let module = translate_to_fuzz(&seed);

            assert!(module.is_valid());
        }
    }

    #[test]
    fn test_translate_to_fuzz_mvp() {
        let mut seed = vec![0; 1000];
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut seed);
            let module = translate_to_fuzz_mvp(&seed);

            assert!(module.is_valid());
        }
    }

    #[test]
    fn test_mutate_to_fuzz() {
        let mut module = Module::new();

        let mut seed = vec![0; 1000];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut seed);
        mutate_to_fuzz(&mut module, &seed);

        assert!(module.is_valid());
    }
}
