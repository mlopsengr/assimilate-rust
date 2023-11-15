#[cfg(feature = "tch-gpu")]
mod tch_gpu {
    use burn_autodiff::Autodiff;
    use burn_tch::{Libtorch, LibTorchDevice};
    use mnist::training;

    pub fn run() {
        #[cfg(not(target_os = "macos"))]
        let device = LibTorchDevice::cuda(0);

    }
    
}
