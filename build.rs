extern crate embed_resource;

fn main() {
    embed_resource::compile("resources.rc");
    windows::build! {
        Windows::Win32::Foundation::*,
        Windows::Foundation::Uri,
        Windows::System::LauncherOptions,
        Windows::System::Launcher
    };
}
