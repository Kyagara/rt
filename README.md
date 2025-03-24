# RT (name pending)

A Twitch frontend written in Rust using Tauri and SvelteKit.

<table>
    <tr>
        <td align="center">
            <img alt="zfg1 playing OOT Randomizer" src=".github/assets/screenshot-1.jpg" width="500">
            <em>
                <div>
                    <a href="https://www.twitch.tv/zfg1">zfg1</a> playing OOT. Chat on Copium for Prime 4.
                </div>
            </em>
        </td>
        <td align="center">
            <img alt="paganmars playing Monster Hunter Wilds" src=".github/assets/screenshot-2.jpg" width="500">
            <em>
                <div>
                    <a href="https://www.twitch.tv/paganmars">paganmars</a> playing Monster Hunter Wilds. An excelent use of screen space with PiP.
                </div>
            </em>
        </td>
    </tr>
</table>

> This is not meant to be a replacement for the official Twitch app/site, some features (account login, send chat messages, etc) are not implemented and not in the scope of this project.

## Features

- Watch streams in any of the available resolutions.
- View chat with 7tv and BetterTTV emotes support.
- Ad blocking.
- Get viewer count, stream title, current game, hours streamed.
- Replace `https` with `rt` in a stream URL to open the app and save the user.

## Download

> Only installers are available for now and all (`exe`, `deb`, `rpm`) are provided in the same zip file.

[Quick download.](https://nightly.link/Kyagara/rt/workflows/build.yaml/main/bundles.zip)

Github Actions builds are available [here](https://github.com/Kyagara/rt/actions).

## About

> Tested on Windows, other desktops should work but haven't been tested.

- Frontend `src`: [SvelteKit](https://svelte.dev/docs/kit/introduction), [tailwindcss](https://tailwindcss.com).
- Backend `src-tauri`: [Tauri](https://tauri.app/).

Data is stored in the following locations:

- Windows: `%AppData%/com.rt.app`
- Linux: `~/.config/com.rt.app`

Logs are stored in the following locations:

- Linux: `$XDG_DATA_HOME/com.rt.app/logs` or `$HOME/.local/share/com.rt.app/logs`
- Windows: `%LocalAppData%/com.rt.app/logs`

## TODO

- Maybe cache users/emotes/feeds in the AppState, also maybe return them when possible in the same request instead of emitting an update event.
- Put the seek bar at the end when joining a stream, currently it is some seconds behind when first joining.
- Improve layout (add information about the content somewhere in the watch page).
- Fix titlebar view name not being set correctly when opening new watch pages from YouTube.
- Youtube:
  - Maybe move from youtube embed to using RustyPipe's botguard and retrieve video URLs from there.
  - Add YouTube channel page with video search.
  - Allow downloading videos/thumbnails.
  - Subscribe to channel from watch page.
  - Add pagination to videos list.
- More logging and better errors.
- Add global Twitch emotes.
