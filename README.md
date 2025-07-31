# 🎵 Media Downloader
Easy download music and videos from any web source such as youtube/facebook/reddit/etc...

✨ Tested And Working on:
- Facebook
- Youtube and Playlists
- Youtube Music and Playlists
- Reddit
- Twitter

### 🔧 Requirements
- ⚙️ yt-dlp (CLI tool)
- 🎬 ffmpeg (for audio/video conversion)

# 💻 Installation Windows
Windows users needs to download [ffmpeg](https://ffmpeg.org/download.html) and [yt-dlp](https://github.com/yt-dlp/yt-dlp/releases), put both programs inside media_downloader/libraries

# 🐧 Installation Windows
Install ffmpeg and yt-dlp from your package manager

# 🚀 Usage
Opening executable is the easy and friendly way

⚙️ Additional Arguments:
- --ytdlpPath:	Path to yt-dlp binary
- --ffmpegPath:	Path to ffmpeg binary
- --resultFolder:	Folder where the downloaded file will be saved
- --quality:	Download quality: high (default), medium, or low
- --extension:	Output file extension (e.g., mp3, mp4, m4a, etc.)

---

### Full Example Linux:
``./media_downloader --ydlpPath "./libraries/yt-dlp" --ffmpegPath "./libraries/ffmpeg" --resultFolder "/home/user/Videos" --quality high --extension mp4 "https://youtube.com/insanevideo``

### Full Example Windows:
``./media_downloader --ydlpPath "./libraries/yt-dlp.exe" --ffmpegPath "./libraries/ffmpeg.exe" --resultFolder "C:\Users\leandro.schmidt\Videos" --quality high --extension mp4 "https://youtube.com/insanevideo``

---

## 📦 Building
```bash
git clone https://github.com/LeandroTheDev/media_downloader
cd media_downloader
cargo build --release
```