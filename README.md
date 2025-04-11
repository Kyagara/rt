# RT (name pending)

> [!WARNING]
> WIP! YouTube videos will run into 403 after some time, use embedded videos instead.

A Twitch and YouTube frontend using Svelte and Electron.

<table>
    <tr>
        <td align="center">
            <img alt="vinesauce playing Yoshi's Island" src=".github/assets/screenshot-1.webp" width="500">
            <em>
                <div>
                    <a href="https://www.twitch.tv/vinesauce">vinesauce</a> playing Yoshi's Island.
                </div>
            </em>
        </td>
        <td align="center">
            <img alt="YouTube feed" src=".github/assets/screenshot-2.webp" width="500">
            <em>
                <div>
                    YouTube feed with videos and streams.
                </div>
            </em>
        </td>
    </tr>
</table>

## Features

- Import YouTube subscriptions. (Accepts a `csv` file separated by Channel ID, URL, Title)
- Add users to your stream and videos feeds.
- Watch content in any of the available resolutions.
- View Twitch chat with 7tv and BetterTTV emotes.
- Block ads.
- Open videos or streams directly in the app using `rt:://` URLs.
- Picture-in-picture (except for embedded videos).

## About

Tested on Windows and Ubuntu. Not tested on macOS.

### Download

> These are files from the latest successful build, they do not require a GitHub account to download.

- [Windows](https://nightly.link/Kyagara/rt/workflows/build.yaml/electron/windows.zip)
- [Linux](https://nightly.link/Kyagara/rt/workflows/build.yaml/electron/linux.zip)
- [macOS](https://nightly.link/Kyagara/rt/workflows/build.yaml/electron/mac.zip)

Github Actions artifacts are available [here](https://github.com/Kyagara/rt/actions).

### Redirects

On launch, a custom protocol handler is registered for `rt://` URLs, this allows you to open videos or streams directly in the app.

If the app is not running, it will be started with the URL as an argument, if it is running, the URL will be opened in a new window.

`YouTube`:

- `rt://yt/dQw4w9WgXcQ`
- `rt://youtube/dQw4w9WgXcQ`
- `rt://www.youtube.com/watch?v=dQw4w9WgXcQ`
- `rt://youtu.be/dQw4w9WgXcQ`

`Twitch`:

- `rt://tw/zfg1`
- `rt://twitch/zfg1`
- `rt://www.twitch.tv/zfg1`

If you are using extensions like [LibRedirect](https://github.com/libredirect/browser_extension), you can set a frontend for YouTube like Invidious and set the instance URL to `rt://`. The same can be done for Twitch, you can set the frontend to SafeTwitch and set the instance URL to `rt://`.

### Paths

To store users, feeds and emotes, SQLite is used with [better-sqlite3](https://github.com/WiseLibs/better-sqlite3).

Data (databases, window state, etc):

- Windows: `%AppData%/com.rt.app`
- Linux: `~/.config/com.rt.app`

Logs:

- Windows: `%LocalAppData%/com.rt.app/logs`
- Linux: `$XDG_DATA_HOME/com.rt.app/logs` or `$HOME/.local/share/com.rt.app/logs`

### Frontends

`YouTube`:  

The feed uses YouTube's rss feed to retrieve videos to avoid rate limits, this sadly does not contain video duration or if the content is a livestream.

The watch page will try to retrieve a YouTube player using [YouTube.js](https://github.com/LuanRT/YouTube.js), if it fails, it will use Vidstack's YouTube [provider](https://vidstack.io/docs/player/api/providers/youtube/) to play videos via embeds, this fallback has the drawbacks of not being able to play videos that disallows embedding and not being able to select a video quality. You have the option to switch between them.

`Twitch`:

The player uses a custom [hls.js](https://github.com/video-dev/hls.js/) loader that fetches and reads the playlists, this is what allows for ad blocking as the loader can detect ads and switch to a backup stream until ads are over, this was inspired by [TwitchAdSolutions](https://github.com/pixeltris/TwitchAdSolutions) method of switching streams.

Uses GQL queries from the internal Twitch API to retrieve user data and stream playback.

## TODO

- Tweak tsconfig and maybe add some more linting.
- Themes.
- Error handling.
- Logging.
- Maybe move to using classes.
- Add back `rpm` to bundles.
- Copy watch page content URL.
- YouTube:
  - Fix URLs in descriptions, they currently open the URL in the app instead of the default browser, the links are also not formatted properly.
  - Add YouTube channel page with video search.
  - Allow downloading videos (maybe using `yt-dlp`) and thumbnails.
  - Search for videos in the feed.
- Twitch:
  - Put the seek bar at the end when joining a stream, currently it is some seconds behind when first joining.
  - Button for adding/removing user from the watch page.
