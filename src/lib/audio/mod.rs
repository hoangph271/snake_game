use std::io::Cursor;
use std::sync::{Arc, Mutex};

pub fn play_background_music(is_game_ended: Arc<Mutex<bool>>) {
    use rodio::{source::Source, Decoder, OutputStream};
    use std::io::BufReader;

    if let Ok((_, stream_handle)) = OutputStream::try_default() {
        let binary = include_bytes!("../../bin/8_bit_adventure.mp3").to_vec();
        let binary = Cursor::new(binary);
        let file = BufReader::new(binary);
        let source = Decoder::new_looped(file).unwrap();

        stream_handle.play_raw(source.convert_samples()).unwrap();
        loop {
            // FIXME: Workaround for keeping the background music playing
            if *is_game_ended.lock().unwrap() {
                break;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(10))
            }
        }
    } else {
        println!("Can NOT init sound output...!");
    }
}
