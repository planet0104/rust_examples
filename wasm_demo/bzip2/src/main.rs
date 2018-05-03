extern crate bzip2;

use bzip2::{ Compression, read};
use std::io::Read;
use std::mem;
use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;

thread_local!{
    static BUFFER_PTR: RefCell<*mut u8> = RefCell::new(ptr::null_mut());
}

#[no_mangle]
pub fn decompress(buffer:*mut u8, len: usize) -> usize {
    let buffer = unsafe{ Vec::from_raw_parts(buffer, len, len) };
    let mut decompress_data = bzip2_decompress(&buffer);
    let len = decompress_data.len();
    let p = decompress_data.as_mut_ptr();
    mem::forget(decompress_data);
    BUFFER_PTR.with(|ptr|{
        *ptr.borrow_mut() = p;
    });
    len
}

#[no_mangle]
pub fn get_result_ptr()->*mut u8{
    let mut p:Rc<*mut u8> = Rc::new(ptr::null_mut());
    BUFFER_PTR.with(|ptr|{
        *(Rc::get_mut(&mut p).unwrap()) = *ptr.borrow_mut();
    });
    *p
}

#[no_mangle]
pub fn compress(buffer:*mut u8, len: usize) -> usize {
    let buffer = unsafe{ Vec::from_raw_parts(buffer, len, len) };
    let mut compress_data = bzip2_compress(&buffer);
    let len = compress_data.len();
    let p = compress_data.as_mut_ptr();
    mem::forget(compress_data);
    BUFFER_PTR.with(|ptr|{
        *ptr.borrow_mut() = p;
    });
    len
}

fn bzip2_decompress(buffer: &Vec<u8>) ->Vec<u8>{
    let mut decompressor = read::BzDecoder::new(buffer.as_slice());
    let mut data:Vec<u8> = vec![];
    decompressor.read_to_end(&mut data).unwrap();
    data
}

fn bzip2_compress(buffer: &Vec<u8>) -> Vec<u8>{
    let mut compressor = read::BzEncoder::new(buffer.as_slice(), Compression::Best);
    let mut compress_data = vec![];
    compressor.read_to_end(&mut compress_data).unwrap();
    compress_data
}

fn main() {
    test();
}

#[test]
fn test(){
    let text = "泄密大神 Benjamin Geskin 发推分享两张 iPhone SE 2 的渲染图，据称是根据 3D CAD 文件制作。Geskin 在评论中提到，他本人并不相信 iPhone SE 2 会有前刘海，不过这些图片是根据 3D CAD 文件制作的。从图片中可以看出，这款 iPhone SE 2 将会搭载与 iPhone X 类似全面屏和「刘海」，不过刘海的边缘切角的角度更大一些，刘海也更窄了。四周的边框貌似也比 iPhone X 的边框更窄一些。后面是铝合金金属材质，难道不打算支持无线充电？别做梦了，不可能给se上全面屏的，就算上了价格绝对也不会se真是这样的话必买一台，4寸机身 4.7的屏幕?怎么看?怎么难看。不就合成了一张X的屏幕图到SE上面--iPhone SE 2渲染图欣赏：根据3D CAD文件制作根据对路易威登集团总裁 Bernard Arnault 的最新采访可知，苹果创始人史蒂夫乔布斯在重返苹果后不久曾就苹果品牌直营店问题向其征求意见。虽然这个想法在当时看起来十分疯狂，但乔布斯当时看到了这对奢侈品制造商很受用。我不知道你是否还记得，当时几乎所有苹果的竞争对手都表示开店这事太过疯狂。我记得戴尔是这么说的。很明显，他们完全错了，Apple Store 直营店取得了巨大的成功。」";
    let bytes = text.as_bytes();
    let mut data =  bytes.to_vec();
    println!("{:?}", data);
    //压缩
    let len = compress(data.as_mut_ptr(), data.len());
    let mut compress_data = unsafe{ Vec::from_raw_parts(get_result_ptr(), len, len) };

    //解压缩
    let len = decompress(compress_data.as_mut_ptr(), compress_data.len());
    let decompress_data = unsafe{ Vec::from_raw_parts(get_result_ptr(), len, len) };

    println!("{:?}", decompress_data);
}