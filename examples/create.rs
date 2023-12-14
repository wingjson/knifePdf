/*
 * @Date: 2023-11-03 11:22:48
 * @LastEditors: WWW
 * @LastEditTime: 2023-11-20 16:32:40
 * @FilePath: \lopdf-master\examples\create.rs
 */
// use lopdf::Document;

// fn main() -> Result<(), lopdf::Error> {
//     let doc = Document::load("1.pdf")?;

//     for (object_id, object) in doc.objects.iter() {
//         let object_number = object_id.0;
//         let generation_number = object_id.1;
//         println!("Object {} {}", object_number, generation_number);
//     }

//     Ok(())
// }

// use lopdf::Document;

// fn main() -> Result<(), lopdf::Error> {
//     let doc = Document::load("1.pdf")?;

//     // 检查文档是否包含加密字典
//     let is_encrypted = doc.trailer.get(b"Encrypt").is_ok();

//     println!("Is Document Encrypted? {}", is_encrypted);

//     // 如果需要，可以进一步解析加密字典的细节
//     if is_encrypted {
//         if let Some(encrypt_dict) = doc.trailer.get(b"Encrypt") {
//             // 分析加密字典
//             println!("Encryption Dictionary: {:?}", encrypt_dict);
//         }
//     }

//     Ok(())
// }



use lopdf::Document;
use lopdf::Object;
fn main() -> Result<(), lopdf::Error> {
    // let doc = Document::load("1.pdf")?;
    let doc = Document::watermark_by_file_txt("1.pdf".to_owned(),"机构知识库".to_owned());
    
    // // 检查文档是否包含加密字典
    // let is_encrypted = doc.trailer.get(b"Encrypt").is_ok();

    // println!("Is Document Encrypted? {}", is_encrypted);

    // // 如果需要，可以进一步解析加密字典的细节
    // if is_encrypted {
    //     if let Some(encrypt_dict) = doc.trailer.get(b"Encrypt") {
    //         // 分析加密字典
    //         println!("Encryption Dictionary: {:?}", encrypt_dict);
    //     }
    // }

    Ok(())
}