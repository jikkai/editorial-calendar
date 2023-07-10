use neon::prelude::*;
use svg::node::element::{Rectangle};
use svg::Document;
use std::fs::File;
use std::io::BufWriter;

// 根据值选择相应的颜色
fn get_color(value: u32) -> &'static str {
    match value {
        0 => "#ebedf0",
        1 => "#c6e48b",
        2 => "#7bc96f",
        3 => "#239a3b",
        _ => "#196127",
    }
}

fn draw(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let input = cx.argument::<JsString>(0)?;
    let str = input.value(&mut cx);
    let json_str = str.as_str();
    let data: Vec<u32> = serde_json::from_str(json_str).unwrap();

    // 每个方格的大小为 10x10
    const SQUARE_SIZE: u32 = 10;
    const BORDER_SIZE: u32 = 1;
    const TOTAL_COLS: u32 = 54;
    const TOTAL_ROWS: u32 = 7;
    const WHITE_BORDER_SIZE: u32 = 1;

    // 计算视图框和图像大小
    let viewbox_width = (TOTAL_COLS + WHITE_BORDER_SIZE * 2) * SQUARE_SIZE;
    let viewbox_height = (TOTAL_ROWS + WHITE_BORDER_SIZE * 2) * SQUARE_SIZE;

    // 创建 SVG 文档对象
    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));

    // 绘制白色边框
    let white_border = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", viewbox_width)
        .set("height", viewbox_height)
        .set("fill", "white");
    document = document.add(white_border);

    // 绘制方格和填充颜色
    for (i, value) in data.iter().enumerate() {
        let col = i as u32 / TOTAL_ROWS;
        let row = i as u32 % TOTAL_ROWS;

        let x = col * SQUARE_SIZE + WHITE_BORDER_SIZE * SQUARE_SIZE;
        let y = row * SQUARE_SIZE + WHITE_BORDER_SIZE * SQUARE_SIZE;

        // 绘制方格
        let rectangle = Rectangle::new()
            .set("x", x + BORDER_SIZE)
            .set("y", y + BORDER_SIZE)
            .set("width", SQUARE_SIZE - BORDER_SIZE * 2)
            .set("height", SQUARE_SIZE - BORDER_SIZE * 2)
            .set("fill", get_color(*value)); // 根据值选择相应的颜色
        document = document.add(rectangle);
    }

    // 将 SVG 文档保存到文件
    let file = File::create("contributions.svg").unwrap();
    let writer = BufWriter::new(file);
    svg::write(writer, &document).unwrap();

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("draw", draw)?;
    Ok(())
}
