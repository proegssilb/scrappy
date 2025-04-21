use std::process::exit;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_file_exit(|| {
       exit(0); 
    });
    
    ui.on_menu_click(|cmd| {
       println!("{:?}", cmd); 
    });

    ui.run()
}
