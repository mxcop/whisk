// use std::{thread::{JoinHandle, self}, sync::{atomic::{AtomicBool, AtomicUsize}, Arc, Mutex}, collections::HashMap};

// use anstyle::{Style, AnsiColor};
// use crossterm_cursor::cursor;
// use stream::Streams;

// mod stream;
pub mod color;

// #[derive(Debug, PartialEq)]
// pub enum CompileStatus {
//     Busy,
//     Done,
//     Failed
// }

// #[derive(Debug)]
// pub struct CompileTUI {
//     thread_handle: Option<JoinHandle<()>>,
//     running: Arc<AtomicBool>,
//     compiling: Arc<Mutex<HashMap<String, CompileStatus>>>,
//     last_line_len: Arc<AtomicUsize>
// }

// fn line(file: &String, status: &CompileStatus) -> String {
//     let success_style: Style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into()));
//     let bold_style: Style = Style::new().bold();

//     format!("Compiling {}{}{} ... {}{:?}{}\n",
//         bold_style.render(),
//         file,
//         bold_style.render_reset(),
//         success_style.render(),
//         status,
//         success_style.render_reset()
//     )
// }

// impl CompileTUI {
//     pub fn new(out_name: &String, num_files: u32) -> Self {
//         Self::new_with_stream(out_name.clone(), num_files, Streams::default())
//     }

//     pub fn new_with_stream(out_name: String, num_files: u32, stream: Streams) -> Self {
//         let running = Arc::new(AtomicBool::new(true));
//         let compiling: Arc<Mutex<HashMap<String, CompileStatus>>> = Arc::new(Mutex::new(HashMap::new()));
//         let last_line_len = Arc::new(AtomicUsize::new(0));

//         let mut cursor = cursor();
//         // write!(stream, "\r\n");
        
//         // We use atomic bools to make the thread stop itself when the `spinner.stop()` method is called.
//         let handle = thread::spawn({
//             let running = Arc::clone(&running);
//             let compiling = Arc::clone(&compiling);
//             let last_line_len = Arc::clone(&last_line_len);

//             move || {
//                 let timer = std::time::SystemTime::now();
//                 while running.load(std::sync::atomic::Ordering::Relaxed) {
//                     {
//                         let files = compiling.lock().unwrap();

//                         // Get progress as percentage.
//                         let percent = files.iter().filter(|(_, &ref status)| status.eq(&CompileStatus::Done)).count() as f32 / num_files as f32;
//                         let progress = (percent * 23f32) as i32;

//                         let building_fmt = {
//                             let style = Style::new().fg_color(Some(AnsiColor::BrightCyan.into())).bold();
//                             format!("    {}Building{} [", style.render(), style.render_reset())
//                         };

//                         let mut info_line = building_fmt.clone();
//                         for _ in 0..progress {
//                             info_line.push('=');
//                         }
//                         info_line.push('>');
//                         for _ in progress..23 {
//                             info_line.push(' ');
//                         }
//                         info_line.push(']');

//                         info_line.push_str(&format!(" {}/{}: {}", 
//                             percent * num_files as f32,
//                             num_files,
//                             files.iter().filter(|(_, &ref status)| status == &CompileStatus::Busy).map(|(name, _)| name.clone()).collect::<Vec<String>>().join(", ")
//                         ));

//                         if files.iter().filter(|(_, &ref status)| status == &CompileStatus::Busy).count() <= 0 {
//                             info_line.push_str(&format!("{}(bin)", &out_name));
//                         }

//                         for _ in info_line.len()..last_line_len.load(std::sync::atomic::Ordering::Relaxed) {
//                             info_line.push(' ');
//                         }

//                         write!(stream, "{}", info_line);
//                         cursor.move_left(info_line.len() as u16).unwrap();

//                         last_line_len.store(info_line.chars().count(), std::sync::atomic::Ordering::Relaxed);
//                     }

//                     stream
//                         .get_stream()
//                         .flush()
//                         .expect("error: failed to flush stream");

//                     thread::sleep(std::time::Duration::from_millis(
//                         16u64
//                     ));
//                 }
//                 let time = timer.elapsed().unwrap().as_secs_f32();

//                 //    Finished dev [unoptimized + debuginfo] target(s) in 1.11s
//                 let finished_fmt = {
//                     let style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into())).bold();
//                     format!("    {}Finished{} {} target in {:.2}s", style.render(), style.render_reset(), out_name, time)
//                 };

//                 write!(stream, "{}", finished_fmt);

//                 for _ in finished_fmt.chars().count()..last_line_len.load(std::sync::atomic::Ordering::Relaxed) {
//                     write!(stream, "{}", ' ');
//                 }
//                 print!("\r\n");
//             }
//         });

//         // Return a Spinner struct
//         Self {
//             thread_handle: Some(handle),
//             running,
//             compiling,
//             last_line_len
//         }
//     }

//     pub fn add_file(&self, name: &String) {
//         let compile_fmt = {
//             let style = Style::new().fg_color(Some(AnsiColor::BrightGreen.into())).bold();
//             format!("   {}Compiling{}", style.render(), style.render_reset())
//         };

//         let info = format!("{} {}", &compile_fmt, name);
//         print!("{info}");

//         for _ in info.chars().count()..self.last_line_len.load(std::sync::atomic::Ordering::Relaxed) {
//             print!(" ");
//         }
//         print!("\r\n");

//         self.compiling.lock().unwrap().insert(name.clone(), CompileStatus::Busy);
//     }

//     pub fn set_file(&self, name: &String, status: CompileStatus) {
//         self.compiling.lock().unwrap().insert(name.clone(), status);
//     }

//     pub fn stop(&mut self) {
//         // Set flag to signal thread to stop
//         self.running
//             .store(false, std::sync::atomic::Ordering::Relaxed);

//         // Wait for the thread to actually stop
//         // Also deletes the last line of the terminal after stopped
//         self.thread_handle
//             .take()
//             .expect("Stopping the spinner thread should only happen once.")
//             .join()
//             .expect("Thread to join.");
//     }
// }
