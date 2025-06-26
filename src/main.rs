use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use image::{io::Reader as ImageReader, Rgba};
use palette::Srgb;
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    // ターミナル設定
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // 画像読み込み
    let img = ImageReader::open("image.png")?.decode()?.to_rgba8();
    let (width, height) = img.dimensions();

    // メインループ
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Image Display (4x4 Braille)")
                .borders(Borders::ALL);
            let inner_area = block.inner(size);
            f.render_widget(block, size);

            // デバッグ情報
            let debug_text = format!(
                "Image: {}x{} pixels\nTerminal area: {}x{} cells\nPress 'q' to quit",
                width, height, inner_area.width, inner_area.height
            );
            f.render_widget(
                Paragraph::new(debug_text).block(Block::default().borders(Borders::ALL)),
                Rect::new(inner_area.right().saturating_sub(20), inner_area.top(), 20, 5),
            );

            // ターミナルサイズが小さすぎる場合の警告
            if inner_area.width < (width * 2) as u16 || inner_area.height < (height * 2) as u16 {
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
            if pixel[3] == 0 {
                continue; // 透明ピクセルはスキップ
            }

            // 256色に変換
            let color = rgba_to_256_color(pixel);

            // 4x4グリッドを4つの点字文字（各2x4）で表現
            // 全てのドットをオンにした点字文字（U+28FF）を使用
            let braille_char = '\u{28FF}'; // 全てのドットがオン

            // 2x2のターミナルセルに描画（4x4ピクセルを表現）
            for dy in 0..2 {
                for dx in 0..2 {
                    let tx = cell_x + dx;
                    let ty = cell_y + dy;
                    if tx >= area.width as u32 || ty >= area.height as u32 {
                        continue;
                    }
                    frame.buffer_mut().set_string(
                        area.left() + tx as u16,
                        area.top() + ty as u16,
                        &braille_char.to_string(),
                        Style::default().fg(color),
                    );
                }
            }
        }
    }
}

// RGBAを256色に変換
fn rgba_to_256_color(pixel: &Rgba<u8>) -> Color {
    if pixel[3] == 0 {
        return Color::Reset;
    }
    let r = (pixel[0] as f32 / 51.0).round() as u8; // 0-5
    let g = (pixel[1] as f32 / 51.0).round() as u8; // 0-5
    let b = (pixel[2] as f32 / 51.0).round() as u8; // 0-5
    let color_index = 16 + 36 * r + 6 * g + b;
    Color::Indexed(color_index)
}