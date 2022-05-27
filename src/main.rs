use ndarray as nd;
use opencv as cv2;
use cv_convert::{FromCv, IntoCv, TryFromCv, TryIntoCv};
use opencv::prelude::*;
use anyhow::Result;

trait AsArray {
    fn try_as_array(&self) -> Result<nd::ArrayView3<u8>>;
}
impl AsArray for cv2::core::Mat {
    fn try_as_array(&self) -> Result<nd::ArrayView3<u8>> {
        let bytes = self.data_bytes()?;
        let size = self.size()?;
        let a = nd::ArrayView3::from_shape((size.height as usize, size.width as usize, 3), bytes)?;
        Ok(a)
    }
}

fn main() {
    let path = "./lena.png".to_string(); 
    let img = cv2::imgcodecs::imread(&path, cv2::imgcodecs::IMREAD_COLOR).unwrap(); 
    // error 
    // let arr: nd::Array3<f32> = img.into_cv(); 
    let arr: nd::ArrayView3<u8> = img.try_as_array().unwrap();
    println!("arr shape {:?}", arr.shape()); 
    // error 
    // let img_rec: cv2::core::Mat = arr.try_into_cv().unwrap();
    cv2::highgui::imshow("img", &img).unwrap();
    // cv2::highgui::imshow("reconstruct", &img_rec);  
    cv2::highgui::wait_key(0).unwrap(); 
}


