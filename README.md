# Universal Intel Driver  
A tool for Windows 10/11 that scans your Intel hardware (CPUs, GPUs, network adapters, etc) and installs missing or outdated drivers automatically. Designed for users who want a hassle-free way to keep their Intel devices up-to-date.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-intel-driver/releases) tab on GitHub.  
2. Download the latest `.zip` file for your system.  
3. Unzip the file and run `UniversalIntelDriver.exe`.  

## Usage  
1. **Run the application** after unzipping.  
2. The GUI will open. Click **Scan** to detect your Intel hardware.  
3. Review detected components and click **Install** to update or add drivers.  
4. Restart your system if prompted.  

No configuration is needed. The app works immediately after installation.  

## Features  
- Detects Intel CPUs, integrated/dedicated GPUs, Wi-Fi/Bluetooth adapters, chipsets, storage controllers, and more.  
- Downloads drivers directly from Intel’s servers for reliability.  
- Silent installation mode (run with `--silent` via command line).  
- Automatic backup of old drivers before updating.  
- Checks for driver updates weekly.  
- Logs errors and installation history in `%AppData%\UniversalIntelDriver\logs`.  
- Supports Windows 10, 11, and future versions (64-bit only).  
- Lightweight GUI with dark/light mode matching system settings.  
- Removes obsolete or conflicting drivers during installation.  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install) and Git.  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-intel-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-intel-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
Contributions are currently not accepted. The project is maintained by the author alone.  

## License  
MIT License. See [LICENSE](LICENSE) file for details.