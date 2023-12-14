
// use lopdf::{Document, Object, Stream, Dictionary, content::{Content, Operation}};
// // use lopdf::crypt::{Protection, CryptFilter, CryptDict, Permission};
// use std::iter::FromIterator;

// fn main() -> Result<(), lopdf::Error> {
//     let mut doc = Document::load("1.pdf")?;
//     let watermark_content = "watermark"; // 这是你的水印内容

//     let pages = doc.get_pages();
//     for (_, &page_id) in pages.iter() {
//         let content_data = doc.get_page_content(page_id)?;
//         let mut content = Content::decode(&content_data)?;

//         let mut new_content = Content { operations: vec![] };

//         // 水印内容添加在原始内容之前
//         // 添加水印
//         new_content.operations.push(Operation::new("BT", vec![]));
//         new_content.operations.push(Operation::new("Tf", vec![Object::Name("Helvetica".into()), Object::Integer(24)]));
//         new_content.operations.push(Operation::new("Td", vec![Object::Integer(100), Object::Integer(100)])); // 调整位置
//         new_content.operations.push(Operation::new("Tj", vec![Object::String(watermark_content.bytes().collect(), lopdf::StringFormat::Literal)]));
//         new_content.operations.push(Operation::new("ET", vec![]));

//         // 然后是原始内容
//         new_content.operations.extend(content.operations);

//         let new_content_data = new_content.encode()?;
//         let new_stream = Stream::new(Dictionary::new(), new_content_data);

//         let new_content_id = doc.add_object(Object::Stream(new_stream));

//         if let Object::Dictionary(ref mut dict) = doc.get_object_mut(page_id)? {
//             dict.set("Contents", Object::Reference(new_content_id));
//         }
//     }

//     doc.save("output_with_watermark.pdf")?;
//     Ok(())
// }


// // #[macro_use]
// // extern crate lopdf;
// // use std::fs::File;
// // use std::io::BufReader;

// // use lopdf::content::{Content, Operation};
// // use lopdf::{Document, Object, Stream};
// // use lopdf::Object::Name;
// // use lopdf::*;
// // use std::{io::BufWriter};
// // use std::iter::FromIterator;

// // fn main(){
// //     let mut doc = Document::load("example_with_watermark.pdf").unwrap();
// //     let watermark_content = "水印内容"; // 这是你的水印内容

// //     let pages = doc.get_pages();
// //     for (_, &page_id) in pages.iter() {
// //         let resources = doc.get_page_resources(page_id);
// //         let content_data = doc.get_page_content(page_id).unwrap();

// //         // 解码原始内容
// //         let mut content = Content::decode(&content_data).unwrap();

// //         // 创建一个新的内容流，用于添加水印
// //         let mut new_content = Content { operations: vec![] };

// //         // 添加水印
// //         new_content.operations.push(Operation::new("BT", vec![])); // 开始文本对象
// //         new_content.operations.push(Operation::new("Tf", vec![Object::Name("F1".into()), Object::Integer(24)])); // 设置字体和大小
// //         new_content.operations.push(Operation::new("Td", vec![Object::Integer(100), Object::Integer(600)])); // 移动文本位置
// //         new_content.operations.push(Operation::new("Tj", vec![Object::String(watermark_content.bytes().collect(), lopdf::StringFormat::Literal)])); // 显示文本
// //         new_content.operations.push(Operation::new("ET", vec![])); // 结束文本对象

// //         new_content.operations.extend(content.operations);

// //         // 重新编码页面内容
// //         let new_content_data = new_content.encode().unwrap();

// //         // 创建一个新的Stream对象
// //         let new_stream = Stream::new(Dictionary::new(), new_content_data);

// //         // 更新页面内容
// //         let new_content_id = doc.add_object(Object::Stream(new_stream));
// //         let temp_obj = doc.get_object_mut(page_id).unwrap();
// //         temp_obj.set("Contents", new_content_id);
// //     }

// //     doc.save("output_with_watermark.pdf");
// // }


// //create pdf
// // fn main(){
// //     let mut doc = Document::with_version("1.5");
// //     let pages_id = doc.new_object_id();
    
// //     // 创建一个透明图形状态参数字典
// //     let gs_id = doc.add_object(dictionary! {
// //         "Type" => "ExtGState",
// //         "CA" => 0.1, // 描绘透明度（填充）
// //         "ca" => 0.1, // 描绘透明度（笔触）
// //     });
    
// //     let font_id = doc.add_object(dictionary! {
// //         "Type" => "Font",
// //         "Subtype" => "Type1",
// //         "BaseFont" => "Helvetica",
// //     });

// //     let resources_id = doc.add_object(dictionary! {
// //         "Font" => dictionary! {
// //             "F1" => font_id,
// //         },
// //         "ExtGState" => dictionary! { // 添加透明度参数字典到资源中
// //             "GSWatermark" => gs_id,
// //         },
// //     });

// //     // 创建水印内容，并将其旋转45度
// //     let watermark_content = Content {
// //         operations: vec![
// //             Operation::new("q", vec![]), // 保存图形状态
// //             Operation::new("BT", vec![]), // 开始文本对象
// //             Operation::new("Tf", vec!["F1".into(), 24.into()]), // 设置字体和字体大小
// //             Operation::new("Tm", vec![0.707.into(), 0.707.into(), (-0.707).into(), 0.707.into(), 200.into(), 300.into()]), // 设置文本矩阵进行旋转
// //             Operation::new("gs", vec![Name("GSWatermark".into())]), // 应用透明度参数
// //             Operation::new("Tj", vec![Object::string_literal("WATERMARK")]), // 设置水印文本
// //             Operation::new("ET", vec![]), // 结束文本对象
// //             Operation::new("Q", vec![]), // 恢复图形状态
// //         ],
// //     };
// //     let watermark_content_id = doc.add_object(Stream::new(dictionary! {}, watermark_content.encode().unwrap()));

// //     // 创建页面内容
// //     let content = Content {
// //         operations: vec![
// //             Operation::new("BT", vec![]),
// //             Operation::new("Tf", vec!["F1".into(), 48.into()]),
// //             Operation::new("Td", vec![100.into(), 600.into()]),
// //             Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
// //             Operation::new("ET", vec![]),
// //         ],
// //     };
// //     let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));;

// //     let page_id = doc.add_object(dictionary! {
// //         "Type" => "Page",
// //         "Parent" => pages_id,
// //         "Contents" => Object::Array(vec![watermark_content_id.into(), content_id.into()]), // 注意水印内容在原始内容之前
// //         "Resources" => resources_id,
// //         "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
// //     });
// //     // 创建页面树
// //     let pages = dictionary! {
// //         "Type" => "Pages",
// //         "Kids" => vec![page_id.into()],
// //         "Count" => 1,
// //         "Resources" => resources_id,
// //         "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
// //     };
// //     doc.objects.insert(pages_id, Object::Dictionary(pages));

// //     // 创建目录对象
// //     let catalog_id = doc.add_object(dictionary! {
// //         "Type" => "Catalog",
// //         "Pages" => pages_id,
// //     });

// //     // 设置文档目录和保存
// //     doc.trailer.set("Root", catalog_id);
// //     doc.compress();

// //     // 保存PDF文件
// //     let mut output = BufWriter::new(File::create("example_with_watermark.pdf").unwrap());
// //     let _ = doc.save_to(&mut output);
// // }
