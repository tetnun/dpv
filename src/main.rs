use anyhow::{Result, Context};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use image::{io::Reader as ImageReader, Rgba};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::env;

fn main() -> Result<()> {
    // コマンドライン引数の取得
    let args: Vec<String> = env::args().collect();
    let image_path = args.get(1)
        .context("Please specify an image file as a command-line argument (e.g., `cargo run image.png`)")?;

    // ターミナル設定
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // 画像読み込み
    let img = ImageReader::open(image_path)
        .with_context(|| format!("Failed to open image file: {}", image_path))?
        .decode()
        .with_context(|| format!("Failed to decode image file: {}", image_path))?
        .to_rgba8();
    let (width, height) = img.dimensions();
    let title = format!("Image: {}x{} pixels. File: {}. Press 'q' to quit", width, height, image_path);

    // メインループ
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(title.to_string())
                .borders(Borders::NONE);
            let inner_area = block.inner(size);
            f.render_widget(block, size);

            // デバッグ情報
            // let debug_text = format!(
            //     "Image: {}x{} pixels\nTerminal area: {}x{} cells\nPress 'q' to quit",
            //     width, height, inner_area.width, inner_area.height
            // );
            // f.render_widget(
            //     Paragraph::new(debug_text).block(Block::default().borders(Borders::ALL)),
            //     Rect::new(inner_area.right().saturating_sub(20), inner_area.top(), 20, 5),
            // );

            // ターミナルサイズが小さすぎる場合の警告
            if inner_area.width < (width * 2) as u16 || inner_area.height < (height / 4) as u16 {
                f.render_widget(
                    Paragraph::new("Warning: Terminal too small!").style(Style::default().fg(Color::Red)),
                    Rect::new(inner_area.left(), inner_area.bottom().saturating_sub(1), inner_area.width, 1),
                );
            }

            // 画像を点字で描画
            render_image(&img, width, height, f, inner_area);
        })?;

        // キー入力処理（'q'で終了）
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // ターミナル復元
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
// 画像を点字文字でターミナルに描画（1ピクセルを4x4で表現）
fn render_image(img: &image::RgbaImage, width: u32, height: u32, frame: &mut Frame, area: Rect) {
    for y in 0..height {
        for x in 0..width {
            // ターミナルセル座標（2x2セルで1ピクセルを表現）
            let cell_x = x * 2;
            let cell_y = y;
            if cell_x >= area.width as u32 || cell_y >= area.height as u32 {
                continue; // 描画エリア外はスキップ
            }

            // ピクセル取得
            let pixel = img.get_pixel(x, y);

            // 256色に変換
            let color = rgba_to_256_color(pixel);
            if color == Color::Reset {
                continue;
            }

            // 4x4グリッドを4つの点字文字（各2x4）で表現
            // 全てのドットをオンにした点字文字（U+28FF）を使用
            let braille_char_on = '\u{28FF}'; // 全てのドットがオン
            let braille_char_off = '\u{2800}'; // 全てのドットがオフ
            let braille_char = if color == Color::Black {
                braille_char_off
            } else {
                braille_char_on
            };

            // 2x2のターミナルセルに描画（4x4ピクセルを表現）
            for dx in 0..2 {
                let tx = cell_x + dx;
                let ty = cell_y;
                if tx >= area.width as u32 || ty >= area.height as u32 {
                    continue;
                }
                frame.buffer_mut().set_string(
                    area.left() + tx as u16,
                    area.top() + ty as u16,
                    &braille_char.to_string(),
                    Style::default().fg(color).bg(Color::Reset),
                );
            }
        }
    }
}

// RGBAを256色に変換
fn rgba_to_256_color(pixel: &Rgba<u8>) -> Color {
    let threshold: u8 = 0xff;
    if pixel[3] == 0 {
        return Color::Reset;
    }
    if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
        return Color::Black;
    }
    if pixel[0] >= threshold && pixel[1] == 0 && pixel[2] == 0 {
        return Color::Red;
    }
    if pixel[0] == 0 && pixel[1] >= threshold && pixel[2] == 0 {
        return Color::Green;
    }
    if pixel[0] == 0 && pixel[1] == 0 && pixel[2] >= threshold {
        return Color::Blue;
    }
    if pixel[0] >= threshold && pixel[1] >= threshold && pixel[2] == 0 {
        return Color::Yellow;
    }
    if pixel[0] == 0 && pixel[1] >= threshold && pixel[2] >= threshold {
        return Color::Cyan;
    }
    if pixel[0] >= threshold && pixel[1] == 0 && pixel[2] >= threshold {
        return Color::Magenta;
    }
    if pixel[0] >= threshold && pixel[1] >= threshold && pixel[2] >= threshold {
        return Color::White;
    }
    let r = (pixel[0] as f32 / 51.0).round() as u8; // 0-5
    let g = (pixel[1] as f32 / 51.0).round() as u8; // 0-5
    let b = (pixel[2] as f32 / 51.0).round() as u8; // 0-5
    let color_index = 16 + 36 * r + 6 * g + b;
    Color::Indexed(color_index)
}