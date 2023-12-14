/*
 * @Date: 2023-11-08 16:55:25
 * @LastEditors: WWW
 * @LastEditTime: 2023-11-20 17:07:10
 * @FilePath: \lopdf-master\src\watermarker.rs
 */
use super::content::{Content, Operation};
use super::{Document, Object, Stream,StringFormat};
use super::*;
use super::Object::Name;

impl Document {
    pub fn watermark_by_file_txt(path:String,txt:String) {
        let mut doc = Document::load(path).unwrap();
        let watermark_content = txt; // 这是你的水印内容
        let pages = doc.get_pages();
        for (_, &page_id) in pages.iter() {
            // let resources = doc.get_page_resources(page_id);
            let content_data = doc.get_page_content(page_id).unwrap();
            // 解码原始内容
            let content = Content::decode(&content_data).unwrap();

            // create new stream
            let mut new_content = Content { operations: vec![] };
            // Make sure the watermarker to be at the top
            new_content.operations.extend(content.operations);
            new_content.operations.push(Operation::new("BT", vec![]));
            // set font and font size
            new_content.operations.push(Operation::new("Tf", vec![Object::Name("Helvetica".into()), Object::Integer(30)]));
            //set watermark position
            new_content.operations.push(Operation::new("Td", vec![Object::Integer(100), Object::Integer(600)]));
            // set angle
            new_content.operations.push(Operation::new("Tm", vec![0.707.into(), 0.707.into(), (-0.707).into(), 0.707.into(), 200.into(), 300.into()]));
            new_content.operations.push(Operation::new("gs", vec![Name("GSWatermark".into())]));
            new_content.operations.push(Operation::new("Tj", vec![Object::String(watermark_content.bytes().collect(), StringFormat::Literal)])); // 显示文本
            new_content.operations.push(Operation::new("ET", vec![])); // 结束文本对象

            // 重新编码页面内容
            let new_content_data = new_content.encode().unwrap();

            // 创建一个新的Stream对象
            let new_stream = Stream::new(Dictionary::new(), new_content_data);

            // 更新页面内容
            let new_content_id = doc.add_object(Object::Stream(new_stream));
            let temp_obj = doc.get_object_mut(page_id).unwrap();
            // temp_obj.set("Contents", new_content_id);
            // 假设 temp_obj 是一个 Object 类型
            match temp_obj {
                Object::Dictionary(ref mut dict) => {
                    // 如果是 Dictionary 类型，则对其进行操作
                    dict.set("Contents", new_content_id);
                },
                _ => {
                    // 处理其他类型的情况或报错
                    println!("Error: Expected a dictionary object");
                }
            }

        }

        doc.save("output_with_watermark.pdf");
    }

}
