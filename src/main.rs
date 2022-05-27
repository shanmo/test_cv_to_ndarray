use cv_convert::{FromCv, IntoCv, TryFromCv, TryIntoCv};
use ndarray as nd;
use opencv as cv2;

fn main() {
    let path = "./lena.png".to_string(); 
    let img = cv2::imgproc::imread(path, cv2::imgproc::IMREAD_COLOR).unwrap(); 
    let arr = img.try_from_cv(); 
    let img_rec = arr.into_cv(); 
    cv2::imgproc::imshow("img", &img_rec);
    cv2::imgproc::imshow("reconstruct", &img_rec);  
    cv2::wait_key(0); 
}
