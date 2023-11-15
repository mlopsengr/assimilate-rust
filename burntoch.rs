#[cfg(feature = "tch-gpu")]
mod tch_gpu {
    use burn_autodiff::Autodiff;
    use burn_tch::{Libtorch, LibTorchDevice};
    use mnist::training;

    pub fn run() {
        #[cfg(not(target_os = "macos"))]
        let device = LibTorchDevice::cuda(0);
        #[cfg(target_os = "macos")]
        let device = LibTorchDevice::Mps;
    
        training::run::<Autodiff<Libtorch<f32>>>(device);
    }    
}

#[cfg(feature = "tch-cpu")]
mod tch_cpu {
    use burn_autodiff::Autodiff;
    use burn_tch::{LibTorch, LibTorchDevice};
    use mnist::training;


}
